#include <chrono>
#include <SDL2/SDL.h>
#include <SDL2/SDL_mixer.h>
#include <string>
#include "Constants.h"

#include "Vec2.h"
#include "Paddle.h"
#include "Ball.h"
#include "Contact.h"
#include "PlayerScore.h"

int main(int argc, char *argv[]) {
	// Initialize SDL components
	SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO);
	TTF_Init();
	Mix_OpenAudio(44100, MIX_DEFAULT_FORMAT, 2, 2048);

	SDL_Window* window = SDL_CreateWindow("Leafie Pong", 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, SDL_WINDOW_SHOWN);
	SDL_Renderer* renderer = SDL_CreateRenderer(window, -1, 0);

	// Initialize the font
	TTF_Font* scoreFont = TTF_OpenFont("DejaVuSansMono.ttf", 40);

	// Initialize sound effects
	Mix_Chunk* wallHitSound = Mix_LoadWAV("WallHit.wav");
	Mix_Chunk* paddleHitSound = Mix_LoadWAV("PaddleHit.wav");

	// Game logic
	{
		// Create the player score text fields
		PlayerScore playerOneScoreText(Vec2(WINDOW_WIDTH / 4, 20), renderer, scoreFont);

		PlayerScore playerTwoScoreText(Vec2(3 * WINDOW_WIDTH / 4, 20), renderer, scoreFont);

		// Create the ball
		Ball ball(Vec2(WINDOW_WIDTH / 2.0f, WINDOW_HEIGHT / 2.0f), Vec2(BALL_SPEED, 0.0f));

		// Create the paddles
		Paddle paddleOne(Vec2(50.0f, WINDOW_HEIGHT / 2.0f), Vec2(0.0f, 0.0f));
		Paddle paddleTwo(Vec2(WINDOW_WIDTH - 50.0f, WINDOW_HEIGHT / 2.0f), Vec2(0.0f, 0.0f));

		int playerOneScore = 0;
		int playerTwoScore = 0;

		bool running = true;
		bool buttons[4] = {};

		float dt = 0.0f;

		while (running)	{
			auto startTime = std::chrono::high_resolution_clock::now();

			SDL_Event event;
			while (SDL_PollEvent(&event))	{
				if (event.type == SDL_QUIT)	{
					running = false;
				}	else if (event.type == SDL_KEYDOWN)	{
					if (event.key.keysym.sym == SDLK_ESCAPE) {
						running = false;
					}	else if (event.key.keysym.sym == SDLK_w) {
						buttons[Buttons::PaddleOneUp] = true;
					} else if (event.key.keysym.sym == SDLK_s) {
						buttons[Buttons::PaddleOneDown] = true;
					}	else if (event.key.keysym.sym == SDLK_UP)	{
						buttons[Buttons::PaddleTwoUp] = true;
					}	else if (event.key.keysym.sym == SDLK_DOWN)	{
						buttons[Buttons::PaddleTwoDown] = true;
					}
				}	else if (event.type == SDL_KEYUP)	{
					if (event.key.keysym.sym == SDLK_w)	{
						buttons[Buttons::PaddleOneUp] = false;
					}	else if (event.key.keysym.sym == SDLK_s) {
						buttons[Buttons::PaddleOneDown] = false;
					}	else if (event.key.keysym.sym == SDLK_UP)	{
						buttons[Buttons::PaddleTwoUp] = false;
					}	else if (event.key.keysym.sym == SDLK_DOWN)	{
						buttons[Buttons::PaddleTwoDown] = false;
					}
				}
			}

			if (buttons[Buttons::PaddleOneUp]) {
				paddleOne.velocity.y = -PADDLE_SPEED;
			} else if (buttons[Buttons::PaddleOneDown]) {
				paddleOne.velocity.y = PADDLE_SPEED;
			} else {
				paddleOne.velocity.y = 0.0f;
			}

			if (buttons[Buttons::PaddleTwoUp]) {
				paddleTwo.velocity.y = -PADDLE_SPEED;
			} else if (buttons[Buttons::PaddleTwoDown]) {
				paddleTwo.velocity.y = PADDLE_SPEED;
			} else {
				paddleTwo.velocity.y = 0.0f;
			}

			// Update the paddle positions
			paddleOne.Update(dt);
			paddleTwo.Update(dt);

			// Update the ball position
			ball.Update(dt);

			// Check collisions
			if (Contact contact = CheckPaddleCollision(ball, paddleOne); contact.type != CollisionType::None) {
				ball.CollideWithPaddle(contact);
				Mix_PlayChannel(-1, paddleHitSound, 0);
			} else if (contact = CheckPaddleCollision(ball, paddleTwo); contact.type != CollisionType::None) {
				ball.CollideWithPaddle(contact);
				Mix_PlayChannel(-1, paddleHitSound, 0);
			} else if (contact = CheckWallCollision(ball); contact.type != CollisionType::None) {
				ball.CollideWithWall(contact);

				if (contact.type == CollisionType::Left) {
					++playerTwoScore;

					playerTwoScoreText.SetScore(playerTwoScore);
				} else if (contact.type == CollisionType::Right) {
					++playerOneScore;

					playerOneScoreText.SetScore(playerOneScore);
				} else {
					Mix_PlayChannel(-1, wallHitSound, 0);
				}
			}

			// Clear the window to black
			SDL_SetRenderDrawColor(renderer, 0x0, 0x0, 0x0, 0xFF);
			SDL_RenderClear(renderer);

			// Set the draw color to be white
			SDL_SetRenderDrawColor(renderer, 0xFF, 0xFF, 0xFF, 0xFF);

			// Draw the net
			for (int y = 0; y < WINDOW_HEIGHT; ++y) {
				if (y % 5) {
					SDL_RenderDrawPoint(renderer, WINDOW_WIDTH / 2, y);
				}
			}

			// Draw the ball
			ball.Draw(renderer);

			// Draw the paddles
			paddleOne.Draw(renderer);
			paddleTwo.Draw(renderer);

			// Display the scores
			playerOneScoreText.Draw();
			playerTwoScoreText.Draw();

			// Present the backbuffer
			SDL_RenderPresent(renderer);

			// Calculate frame time
			auto stopTime = std::chrono::high_resolution_clock::now();
			dt = std::chrono::duration<float, std::chrono::milliseconds::period>(stopTime - startTime).count();
		}
	}

	Mix_FreeChunk(wallHitSound);
	Mix_FreeChunk(paddleHitSound);
	SDL_DestroyRenderer(renderer);
	SDL_DestroyWindow(window);
	TTF_CloseFont(scoreFont);
	Mix_Quit();
	TTF_Quit();
	SDL_Quit();

	return 0;
}
