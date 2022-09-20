using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;

namespace HelloMonogame;

public class Game1 : Game
{
    private GraphicsDeviceManager _graphics;
    private SpriteBatch _spriteBatch;
    private Texture2D _box;
    private int _x;
    private int _y;

    public Game1()
    {
        _graphics = new GraphicsDeviceManager(this);
        Content.RootDirectory = "Content";
        IsMouseVisible = true;
        _x = 0;
        _y = 0;
    }

    protected override void Initialize()
    {
        _box = new Texture2D(GraphicsDevice, 1, 1, false, SurfaceFormat.Color);
        _box.SetData(new[] { Color.White });

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

        var kbState = Keyboard.GetState();
        if (kbState.IsKeyDown(Keys.Left)) {
            _x = _x - 1;
        }
        if (kbState.IsKeyDown(Keys.Right)) {
            _x = _x + 1;
        }
        if (kbState.IsKeyDown(Keys.Up)) {
            _y = _y - 1;
        }
        if (kbState.IsKeyDown(Keys.Down)) {
            _y = _y + 1;
        }

        // TODO: Add your update logic here

        base.Update(gameTime);
    }

    private void DrawRect(Rectangle rect, Color color) {
        _spriteBatch.Draw(_box, rect, color);
    }

    protected override void Draw(GameTime gameTime)
    {
        GraphicsDevice.Clear(Color.CornflowerBlue);

        _spriteBatch.Begin();
        DrawRect(new Rectangle { X = _x, Y = _y, Width = 20, Height = 20 }, Color.White);
        _spriteBatch.End();

        base.Draw(gameTime);
    }
}
