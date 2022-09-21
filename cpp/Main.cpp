#include "engine/GameManager.h"

int main(int argc, char* argv[]) {

	QuickSDL::GameManager* game = QuickSDL::GameManager::Instance();

	game->Run();

	QuickSDL::GameManager::Release();
	game = NULL;

	return 0;
}