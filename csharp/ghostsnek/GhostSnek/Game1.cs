﻿using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using MongoDB.Bson;
using MongoDB.Bson.Serialization;
using MongoDB.Bson.Serialization.Attributes;
using MongoDB.Driver;


namespace GhostSnek;

public class GhostSnek : Game
{
    private const int WIDTH = 800;
    private const int HEIGHT = 800;
    private const int GRID_SIZE = 20;
    public const int MAX_X = (WIDTH / GRID_SIZE) - 1;
    public const int MAX_Y = (HEIGHT / GRID_SIZE) - 1;

    private GraphicsDeviceManager _graphics;
    private SpriteBatch _spriteBatch;
    private Texture2D _pixel;

    private Scene _scene = new Scene();
    private Replay _replay = null;
    private string _dbConnStr;
    private MongoClient _dbClient;
    private IMongoCollection<Replay> _replayColl;

    public GhostSnek(string[] args)
    {
        _graphics = new GraphicsDeviceManager(this);
        Content.RootDirectory = "Content";
        IsMouseVisible = true;
        _graphics.PreferredBackBufferWidth = WIDTH;
        _graphics.PreferredBackBufferHeight = HEIGHT;
        _dbConnStr = String.Format("mongodb+srv://{0}:{1}@cluster0.gb1qy3e.mongodb.net/?retryWrites=true&w=majority", args[0], args[1]);
    }

    protected override void Initialize()
    {
        _pixel = new Texture2D(GraphicsDevice, 1, 1, false, SurfaceFormat.Color);
        _pixel.SetData(new[] { Color.White });

        _dbClient = new MongoClient(_dbConnStr);
        _replayColl = _dbClient.GetDatabase("ghostsnek").GetCollection<Replay>("replays");
        _replay = LoadRandomReplay();

        base.Initialize();
    }

    private Replay LoadRandomReplay() {
        return _replayColl.Aggregate()
            .AppendStage<Replay>(new BsonDocument("$sample", new BsonDocument("size", 1)))
            .FirstOrDefault();
    }

    protected override void LoadContent()
    {
        _spriteBatch = new SpriteBatch(GraphicsDevice);
    }

    protected override void Update(GameTime gameTime)
    {
        switch (_scene.Update(gameTime)) {
            case GameState.Waiting: break;
            case GameState.Update:
                if (_replay != null) {
                    _replay.Update();
                }
                break;
            case GameState.Lost:
                _replayColl.InsertOne(_scene.GetReplay());
                _scene = new Scene();
                _replay = LoadRandomReplay();
                break;
            case GameState.Quit:
                Exit();
                break;
        }

        base.Update(gameTime);
    }

    private void DrawRect(int x, int y, int w, int h, Color color) {
        _spriteBatch.Draw(_pixel, new Rectangle(x, y, w, h), color);
    }

    protected override void Draw(GameTime gameTime)
    {
        GraphicsDevice.Clear(Color.Black);

        _spriteBatch.Begin();
        foreach (Point p in _scene.Snek) {
            DrawRect(p.X * GRID_SIZE, p.Y * GRID_SIZE, GRID_SIZE-1, GRID_SIZE-1, Color.White);
        }
        if (_scene.Food != null) {
            var f = _scene.Food.Value;
            DrawRect(f.X * GRID_SIZE + 5, f.Y * GRID_SIZE + 5, GRID_SIZE-10, GRID_SIZE-10, Color.Green);
        }
        if (_replay != null) {
            foreach (Point p in _replay.Snek) {
                DrawRect(p.X * GRID_SIZE, p.Y * GRID_SIZE, GRID_SIZE-1, GRID_SIZE-1, Color.Gray);
            }
        }
        _spriteBatch.End();

        base.Draw(gameTime);
    }
}

public enum Direction {
    Up,
    Down,
    Left,
    Right,
}

class Ticker {
    private TimeSpan _interval;
    private TimeSpan _elapsed = new TimeSpan();

    public Ticker(TimeSpan interval) {
        _interval = interval;
    }

    public bool Update(TimeSpan elapsed) {
        _elapsed += elapsed;
        if (_elapsed > _interval) {
            _elapsed -= _interval;
            return true;
        }
        return false;
    }
}

class Scene {
    public Point? Food { get; private set; }
    public IEnumerable<Point> Snek {
        get {
            return _snek.Body;
        }
    }

    private Snek _snek = new Snek();
    private Direction _dir = Direction.Right;
    private Direction? _nextDir;
    private Ticker _moveTick = new Ticker(new TimeSpan(0, 0, 0, 0, 250));
    private Random _rand = new Random();
    private List<Event> _rec = new List<Event>();

    public Scene() {
        Food = NewFood();
    }

    private Point NewFood() {
        return new Point(_rand.Next(0, GhostSnek.MAX_X), _rand.Next(0, GhostSnek.MAX_Y));
    }

    public GameState Update(GameTime gameTime) {
        var kbState = Keyboard.GetState();
        if (kbState.IsKeyDown(Keys.Escape))
            return GameState.Quit;
        if (kbState.IsKeyDown(Keys.Left) && _dir != Direction.Right) {
            _nextDir = Direction.Left;
        }
        if (kbState.IsKeyDown(Keys.Right) && _dir != Direction.Left) {
            _nextDir = Direction.Right;
        }
        if (kbState.IsKeyDown(Keys.Up) && _dir != Direction.Down) {
            _nextDir = Direction.Up;
        }
        if (kbState.IsKeyDown(Keys.Down) && _dir != Direction.Up) {
            _nextDir = Direction.Down;
        }

        if (_moveTick.Update(gameTime.ElapsedGameTime)) {
            if (_nextDir != null) {
                _dir = _nextDir.Value;
                _nextDir = null;
            }
            if (!_snek.Move(_dir)) {
                return GameState.Lost;
            }
            var ev = new Event(_dir);
            if (_snek.Head == Food) {
                ev.Grew = true;
                Food = NewFood();
                _snek.Grow();
            }
            _rec.Add(ev);
            return GameState.Update;
        } else {
            return GameState.Waiting;
        }
    }

    public Replay GetReplay() {
        return new Replay(_rec);
    }
}

enum GameState {
    Waiting,
    Update,
    Lost,
    Quit,
}

public class Snek {
    private List<Point> _body;

    public IEnumerable<Point> Body {
        get {
            return _body;
        }
    }

    public Point Head {
        get {
            return _body[0];
        }
    }

    public Snek() {
        _body = new List<Point>();
        for (int i = 4; i >= 0; i--) {
            _body.Add(new Point(i, 0));
        }
    }

    public bool Move(Direction dir) {
        // Calculate next head pos
        int dx = 0, dy = 0;
        switch (dir) {
            case Direction.Up:
                dy = -1;
                break;
            case Direction.Down:
                dy = 1;
                break;
            case Direction.Left:
                dx = -1;
                break;
            case Direction.Right:
                dx = 1;
                break;
        }
        var nextHead = new Point(_body[0].X + dx, _body[0].Y + dy);
        if (nextHead.X < 0 || nextHead.X > GhostSnek.MAX_X || nextHead.Y < 0 || nextHead.Y > GhostSnek.MAX_Y) {
            return false;
        }

        // Update body positions and check for collision
        for (int ix = _body.Count-1; ix > 0; ix--) {
            _body[ix] = _body[ix-1];
            if (_body[ix] == nextHead) {
                return false;
            }
        }
        
        // Update head
        _body[0] = nextHead;
        return true;
    }

    public void Grow() {
        _body.Add(_body[_body.Count-1]);
    }
}

class Event {
    public Direction Dir;
    public bool Grew = false;

    public Event(Direction dir) {
        Dir = dir;
    }
}

[BsonIgnoreExtraElements]
class Replay {
    private List<Event> _events;
    private Snek _snek = new Snek();
    private int _ix = 0;

    [BsonIgnore]
    public IEnumerable<Point> Snek {
        get {
            return _snek.Body;
        }
    }
    public IEnumerable<Event> Events {
        get {
            return _events;
        }
    }

    [BsonConstructor]
    public Replay(IEnumerable<Event> events) {
        _events = new List<Event>(events);
    }

    public void Update() {
        if (_ix >= _events.Count) {
            return;
        }
        var ev = _events[_ix];
        _ix += 1;

        _snek.Move(ev.Dir);
        if (ev.Grew) {
            _snek.Grow();
        }
    }
}