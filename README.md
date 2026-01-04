# AI Mood-Based Daily Companion

A simple AI-based application written in Rust that responds to a user's daily mood and provides supportive messages and lifestyle suggestions.

## Project Overview

This project demonstrates basic AI logic applied to personal and lifestyle applications. The application allows users to select their current mood from predefined options, and based on that input, displays an appropriate supportive message and activity suggestion to encourage emotional awareness and positive daily habits.

## Features

- **Mood Selection**: Users can choose from 7 predefined moods: Happy, Sad, Anxious, Excited, Tired, Angry, and Grateful
- **Personalized Responses**: Each mood has multiple supportive messages and activity suggestions
- **Random Selection**: The app randomly selects from available responses for variety
- **Console-Based Interface**: Simple, easy-to-use command-line interface
- **Ethical Design**: Focuses on emotional support and positive habits

## Learning Objectives

- Understand basic AI concepts and decision-making logic
- Learn how AI can be applied to personal and lifestyle use cases
- Understand simple program structure and flow in Rust
- Improve logical thinking and system design skills

## Installation

1. Ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone or navigate to the project directory
3. Build the project: `cargo build --release`

## Usage

Run the application:

```bash
cargo run
```

Follow the prompts to select your current mood and receive personalized support.

## Project Structure

```
.
├── Cargo.toml          # Rust package configuration
├── src/
│   └── main.rs         # Main application source code
└── README.md           # This file
```

## How It Works

1. **Mood Input**: The user selects their current mood from a numbered list
2. **Response Generation**: Based on the selected mood, the system randomly chooses:
   - A supportive message acknowledging their feelings
   - An activity suggestion to help improve their mood or well-being
3. **Display**: The personalized response is displayed to the user

## Example Interaction

```
Welcome to your AI Mood-Based Daily Companion!
How are you feeling today?

1. Happy
2. Sad
3. Anxious
4. Excited
5. Tired
6. Angry
7. Grateful

Enter the number corresponding to your mood: 1

Based on your mood 'Happy':
💙 I'm glad you're feeling happy! Keep spreading that positive energy.
💡 Suggestion: Share your joy with a loved one by calling them.

Remember, your feelings are valid, and taking care of your emotional well-being is important.
Come back anytime for more support!
```

## Dependencies

- `rand`: For random selection of messages and suggestions

## Future Enhancements

- Add more moods and responses
- Implement user history tracking
- Create a GUI interface
- Add mood trends analysis
- Integrate with calendar or reminder systems

## Target Users

- Students learning about AI concepts
- Beginners interested in practical AI applications
- Individuals seeking simple lifestyle and productivity support
- Rust learners looking for practical projects

## License

This project is open-source and available under the MIT License.