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

    private GraphicsDeviceManager _graphics;
    private SpriteBatch _spriteBatch;
    private Texture2D _pixel;

    private List<Point> _snek = new List<Point>();
    private Direction _dir = Direction.Right;
    private Ticker _moveTick = new Ticker(new TimeSpan(0, 0, 1));

    public Game1()
    {
        _graphics = new GraphicsDeviceManager(this);
        Content.RootDirectory = "Content";
        IsMouseVisible = true;
        _graphics.PreferredBackBufferWidth = WIDTH;
        _graphics.PreferredBackBufferHeight = HEIGHT;

        for (int i = 4; i >= 0; i--) {
            _snek.Add(new Point(i, 0));
        }
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

        // TODO: use this.Content to load your game content here
    }

    protected override void Update(GameTime gameTime)
    {
        if (GamePad.GetState(PlayerIndex.One).Buttons.Back == ButtonState.Pressed || Keyboard.GetState().IsKeyDown(Keys.Escape))
            Exit();

        if (_moveTick.Update(gameTime.ElapsedGameTime)) {
            for (int ix = _snek.Count-1; ix > 0; ix--) {
                _snek[ix] = _snek[ix-1];
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
            _snek[0] = new Point(_snek[0].X + dx, _snek[0].Y + dy);
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
        foreach (Point p in _snek) {
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