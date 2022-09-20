using System.Collections.Generic;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;

namespace GhostSnek;

public class Game1 : Game
{
    const int WIDTH = 800;
    const int HEIGHT = 800;
    const int GRID_SIZE = 20;

    private GraphicsDeviceManager _graphics;
    private SpriteBatch _spriteBatch;
    private Texture2D _pixel;

    private List<Point> _snek;

    public Game1()
    {
        _graphics = new GraphicsDeviceManager(this);
        Content.RootDirectory = "Content";
        IsMouseVisible = true;
        _graphics.PreferredBackBufferWidth = WIDTH;
        _graphics.PreferredBackBufferHeight = HEIGHT;

        _snek = new List<Point>();
        for (int i = 0; i < 5; i++) {
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

        // TODO: Add your update logic here

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
