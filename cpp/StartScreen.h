#ifndef StartScreen_h
#define StartScreen_h

#include "engine/InputManager.h"
#include "engine/Texture.h"
#include "engine/GameEntity.h"
#include "engine/Timer.h"

using namespace QuickSDL;

class StartScreen : public GameEntity {

private:
	Timer* mTimer;
	InputManager* mInput;

		//Top bar entities
	GameEntity* mTopBar;
	Texture* mGameTitle;

		// Logo entities
	Texture* mLogo;


		//Play mode entities
	GameEntity* mPlayModes;
	Texture* mOnePlayerMode;
	Texture* mTwoPlayerMode;
	Texture* mCursor;
	Vector2 mCursorStartPos;
	Vector2 mCursorOffset;
	int mSelectedMode;

		//Bottom Bar entities
	GameEntity* mBotBar;
	Texture* mDates;
	Texture* mRights;

		//Screen animation variables

	Vector2 mAnimationStartPos;
	Vector2 mAnimationEndPos;
	float mAnimationTotalTime;
	float mAnimationTimer;
	bool mAnimationDone;

public:
	StartScreen();
	~StartScreen();

	void ResetAnimation();
	int SelectedMode();
	void ChangeSelectedMode(int change);
	void Update();
	void Render();
};

#endif /* StartScreen_h */