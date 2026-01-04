# AI Mood-Based Daily Companion

A simple AI-based application written in Rust that responds to a user's daily mood and provides supportive messages and lifestyle suggestions.

## Project Overview

This project demonstrates basic AI logic applied to personal and lifestyle applications. The application allows users to select their current mood from predefined options, and based on that input, displays an appropriate supportive message and activity suggestion to encourage emotional awareness and positive daily habits.

## Features

- **Mood Selection**: Users can choose from 10 predefined moods: Happy, Sad, Anxious, Excited, Tired, Angry, Grateful, Stressed, Lonely, and Motivated
- **Interactive Support Categories**: After selecting a mood, users choose from 5 different types of support tailored to that emotion
- **Personalized Responses**: Each mood has multiple supportive messages and categorized activity suggestions
- **Random Selection**: Within each category, suggestions are randomly selected for variety
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

1. **Mood Input**: The user selects their current mood from a numbered list of 10 options
2. **Support Category Selection**: Based on the chosen mood, the user selects from 5 different types of support (e.g., for anger: Physical Release, Emotional Processing, etc.)
3. **Personalized Response**: The system provides a supportive message and a specific suggestion from the chosen category
4. **Display**: The personalized response is displayed to the user

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
8. Stressed
9. Lonely
10. Motivated

Enter the number corresponding to your mood: 6

Great! For feeling Angry, here are some types of support you can choose from:

1. Physical Release
2. Emotional Processing
3. Communication
4. Creative Expression
5. Self-Regulation

Enter the number for the type of support you'd like: 1

Based on your mood 'Angry' and choice of 'Physical Release':
💙 Your anger is telling you something important about your needs.
💡 Suggestion: Try physical activity like punching a pillow or going for a run.

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
Made with love ....yeeee!!