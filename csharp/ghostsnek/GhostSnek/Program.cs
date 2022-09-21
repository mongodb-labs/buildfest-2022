namespace Main;

class Program
{
    static void Main(string[] args) {
        using var game = new GhostSnek.GhostSnek(args);
        game.Run();
    }
}