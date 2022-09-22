#include <SDL2/SDL.h>

#include "Vec2.h"
#include "Constants.h"

#include "AtlasManager.h"

#if !defined(PADDLE_H)
#define PADDLE_H

class Paddle {
public:
	Paddle(Vec2 position, Vec2 velocity): position(position), velocity(velocity) {
		rect.x = static_cast<int>(position.x);
		rect.y = static_cast<int>(position.y);
		rect.w = PADDLE_WIDTH;
		rect.h = PADDLE_HEIGHT;
	}

	void Update(float dt)	{
		position += velocity * dt;

		if (position.y < 0)	{
			// Restrict to top of the screen
			position.y = 0;
		}	else if (position.y > (WINDOW_HEIGHT - PADDLE_HEIGHT)) {
			// Restrict to bottom of the screen
			position.y = WINDOW_HEIGHT - PADDLE_HEIGHT;
		}
	}

	void RecordChange() {

	}

	void Draw(SDL_Renderer* renderer)	{
		if (RecordChanges && rect.y != static_cast<int>(position.y)) {
			AtlasManager::Instance()->WritePlayerMove(position, velocity);
		}
		rect.y = static_cast<int>(position.y);

		SDL_RenderFillRect(renderer, &rect);
	}

	Vec2 position;
	Vec2 velocity;
	bool RecordChanges = false;
	SDL_Rect rect{};
};

#endif