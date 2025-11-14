# GUI Features

## Overview

The R Games Launcher GUI provides a modern, Epic Games Store-inspired interface for managing your game library. Built with egui/eframe, it offers a native, cross-platform experience with minimal dependencies.

## Login Screen

The login screen is the first thing users see when launching the GUI:

- **Epic Games Branding**: Displays "Epic Games Store" title
- **Sign-In Form**: 
  - Email address input field
  - Password input field (masked)
  - Sign In button (enabled only when both fields are filled)
- **Status Messages**: Shows authentication progress and errors
- **Dark Theme**: Uses Epic Games-inspired dark color scheme
- **Demo Note**: Informs users that full OAuth is not yet implemented

## Library View

Once authenticated, users are presented with their game library:

### Top Bar
- **Search Box**: Filter games by title in real-time
- **Filter Buttons**: 
  - "All Games": Shows all games in library
  - "Installed": Shows only installed games

### Game Grid
Games are displayed as cards in a responsive grid layout:

- **Game Card Components**:
  - Game image placeholder (dark background with centered title)
  - Game title below the image
  - Action buttons based on game state:
    - **Not Installed**: "Install" button
    - **Installing**: "Installing..." button (disabled)
    - **Installed**: "Play" and "Uninstall" buttons

### Status Bar
- Displays operation status messages (install progress, launch status, errors)
- "Clear" button to dismiss messages

### Top Panel
- **Title**: "R Games Launcher"
- **Logout Button**: Returns to login screen and clears session

## Color Scheme

The GUI uses a dark theme inspired by Epic Games Store:

- **Window Background**: RGB(18, 18, 18)
- **Panel Background**: RGB(25, 25, 28)
- **Card Background**: RGB(32, 32, 36)
- **Button Hover**: RGB(60, 60, 65)
- **Active/Selected**: RGB(0, 121, 214) - Epic Blue
- **Text**: RGB(230, 230, 230)

## Functionality

### Authentication (Demo Mode)
Currently implements a demo authentication flow:
- Accepts any email/password combination
- Creates a demo authentication token valid for 24 hours
- Saves the token to disk for persistence
- In production, would use Epic Games OAuth device flow

### Game Library
Demo mode shows 5 sample games:
- Demo Game 1
- Epic Adventure
- Racing Challenge
- Strategy Master
- Space Shooter

In production, would fetch real games from Epic Games API.

### Installation Process
1. User clicks "Install" on a game card
2. Button changes to "Installing..." and is disabled
3. Creates game directory structure
4. Creates a demo executable script
5. Saves installation metadata
6. Updates UI to show "Play" and "Uninstall" buttons
7. Shows completion message in status bar

### Launching Games
1. User clicks "Play" on installed game
2. Executes the game's main executable
3. Shows launch status in status bar
4. Game runs in separate process

### Uninstalling Games
1. User clicks "Uninstall" on installed game
2. Removes game files from disk
3. Removes installation metadata
4. Updates UI to show "Install" button
5. Shows completion message in status bar

## User Experience Features

- **Real-time Search**: Filter games as you type
- **Responsive Layout**: Grid adapts to window size
- **Visual Feedback**: Hover effects on buttons and cards
- **Status Messages**: Clear feedback for all operations
- **Minimal Design**: Clean, uncluttered interface
- **Fast Performance**: Native code with efficient rendering

## Technical Implementation

- **Framework**: egui v0.29 with eframe
- **Threading**: Uses tokio for async operations and std::thread for background tasks
- **State Management**: Arc<Mutex<>> for shared state across threads
- **Persistence**: JSON for auth tokens, installed game metadata
- **Configuration**: TOML for user settings

## Future Enhancements

- Real Epic Games OAuth integration
- Live game library syncing
- Download progress bars
- Game cover art images
- Cloud save management
- Update notifications
- Settings panel
- Multiple user accounts
- Friend list integration
