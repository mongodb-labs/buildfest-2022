using System;
using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;

namespace GhostSnek;

public class Game1 : Game
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

    public Game1()
    {
        _graphics = new GraphicsDeviceManager(this);
        Content.RootDirectory = "Content";
        IsMouseVisible = true;
        _graphics.PreferredBackBufferWidth = WIDTH;
        _graphics.PreferredBackBufferHeight = HEIGHT;
    }

    protected override void Initialize()
    {
        _pixel = new Texture2D(GraphicsDevice, 1, 1, false, SurfaceFormat.Color);
        _pixel.SetData(new[] { Color.White });

        base.Initialize();
    }

    protected override void LoadContent()
    {
        _spriteBatch = new SpriteBatch(GraphicsDevice);
    }

    protected override void Update(GameTime gameTime)
    {
        switch (_scene.Update(gameTime)) {
            case GameState.Playing: break;
            case GameState.Lost:
                _scene = new Scene();
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
        _spriteBatch.End();

        base.Draw(gameTime);
    }
}

enum Direction {
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
    public List<Point> Snek { get; private set; }

    private Direction _dir = Direction.Right;
    private Direction? _nextDir;
    private Ticker _moveTick = new Ticker(new TimeSpan(0, 0, 0, 0, 250));

    public Scene() {
        Snek = new List<Point>();
        for (int i = 4; i >= 0; i--) {
            Snek.Add(new Point(i, 0));
        }
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
            if (!MoveSnek()) {
                return GameState.Lost;
            }
        }
        return GameState.Playing;
    }

    private bool MoveSnek() {
        // Calculate next head pos
        if (_nextDir != null) {
            _dir = _nextDir.Value;
            _nextDir = null;
        }
        int dx = 0, dy = 0;
        switch (_dir) {
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
        var nextHead = new Point(Snek[0].X + dx, Snek[0].Y + dy);
        if (nextHead.X < 0 || nextHead.X > Game1.MAX_X || nextHead.Y < 0 || nextHead.Y > Game1.MAX_Y) {
            return false;
        }

        // Update body positions and check for collision
        for (int ix = Snek.Count-1; ix > 0; ix--) {
            Snek[ix] = Snek[ix-1];
            if (Snek[ix] == nextHead) {
                return false;
            }
        }
        
        // Update head
        Snek[0] = nextHead;
        return true;
    }
}

enum GameState {
    Playing,
    Lost,
    Quit,
}