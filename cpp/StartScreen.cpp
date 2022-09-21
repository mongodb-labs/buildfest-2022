#include "StartScreen.h"
#include "engine/MathHelper.h"

StartScreen::StartScreen() {

	mTimer = Timer::Instance();
	mInput = InputManager::Instance();

	//Top bar entities

	mTopBar = new GameEntity(Vector2(Graphics::Instance()->SCREEN_WIDTH*0.5f, 20.0f));
	mTitle = new Texture("LEAFIE PONG", "emulogic.ttf", 32, {0, 104, 74 });
	mTitle->Parent(mTopBar);
	mTitle->Pos(Vector2(0.0f, 20.0f));
	mTopBar->Parent(this);

	// logo entities
	mLogo = new Texture("dbx-cxx.png", 0, 0, 342, 342);
	mLogo->Pos(Vector2(Graphics::Instance()->SCREEN_WIDTH * 0.5f, Graphics::Instance()->SCREEN_HEIGHT * 0.34f));
	mLogo->Parent(this);

	// Play mode entities
	mPlayModes = new GameEntity(Vector2(Graphics::Instance()->SCREEN_WIDTH*0.5f, Graphics::Instance()->SCREEN_HEIGHT*0.68f));
	mOnePlayerMode = new Texture("1 Player", "emulogic.ttf", 32, { 230, 230, 230 });
	mTwoPlayerMode = new Texture("2 Players", "emulogic.ttf", 32, { 230, 230, 230 });
	mCursor = new Texture("Cursor.png");

	mOnePlayerMode->Parent(mPlayModes);
	mTwoPlayerMode->Parent(mPlayModes);
	mCursor->Parent(mPlayModes);

	mOnePlayerMode->Pos(Vector2( -18.0f, -35.0f));
	mTwoPlayerMode->Pos(Vector2(0.0f, 35.0f));
	mCursor->Pos(Vector2(-175.0f, -35.0f));

	mPlayModes->Parent(this);

	mCursorStartPos = Vector2(mCursor->Pos(local));
	mCursorOffset = Vector2(0.0f, 70.0f);
	mSelectedMode = 0;

	//Bottom bar entities
	mBotBar = new GameEntity(Vector2(Graphics::Instance()->SCREEN_WIDTH*0.5f, Graphics::Instance()->SCREEN_HEIGHT*0.7f));
	mBotBanner2 = new Texture("#BUILDFEST2022-CXX FTW Y'ALL", "emulogic.ttf", 32, { 230, 230, 230 });
	mBotBanner2->Parent(mBotBar);
	mBotBanner2->Pos(Vector2(0.0f, 170.0f));
	mBotBar->Parent(this);

	//Screen animation variables
	ResetAnimation();
}

StartScreen::~StartScreen() {
	//Freeing top bar entities
	delete mTopBar;
	mTopBar = NULL;
	delete mTitle;
	mTitle = NULL;

	//freeing logo entities

	delete mLogo;
	mLogo = NULL;

	//Freeing play mode entities

	delete mPlayModes;
	mPlayModes = NULL;
	delete mOnePlayerMode;
	mOnePlayerMode = NULL;
	delete mTwoPlayerMode;
	mTwoPlayerMode = NULL;
	delete mCursor;
	mCursor = NULL;

	//freeing bottom bar entities

	delete mBotBar;
	mBotBar = NULL;
	delete mBotBanner2;
	mBotBanner2 = NULL;
}

void StartScreen::ResetAnimation() {

	mAnimationStartPos = Vector2( 0.0f, Graphics::Instance()->SCREEN_HEIGHT);
	mAnimationEndPos = VEC2_ZERO;
	mAnimationTotalTime = 2.5f;
	mAnimationTimer = 0.0f;
	mAnimationDone = false;

	Pos(mAnimationStartPos);
}

int StartScreen::SelectedMode() {

	return mSelectedMode;
}

void StartScreen::ChangeSelectedMode(int change) {

	mSelectedMode += change;

	if(mSelectedMode < 0)
		mSelectedMode = 1;
	else if(mSelectedMode > 1 )
		mSelectedMode = 0;

	mCursor->Pos(mCursorStartPos + mCursorOffset * mSelectedMode);

}

void StartScreen::Update() {

	if(!mAnimationDone) {

		mAnimationTimer += mTimer->DeltaTime();
		Pos(Lerp(mAnimationStartPos, mAnimationEndPos, mAnimationTimer / mAnimationTotalTime));

		if(mAnimationTimer >= mAnimationTotalTime) {
			mAnimationDone = true;
		}

		if(mInput->KeyPressed(SDL_SCANCODE_DOWN) || mInput->KeyPressed(SDL_SCANCODE_UP))
			mAnimationTimer = mAnimationTotalTime;
	} else {
		if(mInput->KeyPressed(SDL_SCANCODE_DOWN))
			ChangeSelectedMode(1);
		else if(mInput->KeyPressed(SDL_SCANCODE_UP))
			ChangeSelectedMode(-1);
	}

}

void StartScreen::Render() {
	mTitle->Render();

	mLogo->Render();

	mOnePlayerMode->Render();
	mTwoPlayerMode->Render();
	mCursor->Render();

	mBotBanner2->Render();
}