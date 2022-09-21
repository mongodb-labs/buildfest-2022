#ifndef ScreenManager_h
#define ScreenManager_h

#include "engine/InputManager.h"

class ScreenManager {

private:

	enum SCREENS { start, play };

	static ScreenManager* sInstance;

	QuickSDL::InputManager* mInput;

	// StartScreen* mStartScreen;
	// PlayScreen* mPlayScreen;

	SCREENS mCurrentScreen;

public:
	static ScreenManager* Instance();
	static void Release();

	void Update();
	void Render();

private:

	ScreenManager();
	~ScreenManager();
};

#endif /* ScreenManager_h */