use rand::seq::SliceRandom;
use std::io;

#[derive(Debug, Clone)]
struct MoodResponse {
    messages: Vec<&'static str>,
    suggestions: Vec<&'static str>,
}

fn main() {
    println!("Welcome to your AI Mood-Based Daily Companion!");
    println!("How are you feeling today?");
    println!();

    let moods = vec![
        ("happy", MoodResponse {
            messages: vec![
                "I'm glad you're feeling happy! Keep spreading that positive energy.",
                "Happiness is contagious! Enjoy this wonderful feeling.",
                "Your happiness lights up the world around you."
            ],
            suggestions: vec![
                "Share your joy with a loved one by calling them.",
                "Take a moment to journal what made you happy today.",
                "Go for a walk in nature to amplify your good mood."
            ]
        }),
        ("sad", MoodResponse {
            messages: vec![
                "I'm here for you. It's okay to feel sad sometimes.",
                "Sadness is a normal emotion. Give yourself time to process it.",
                "Remember that this feeling will pass, and brighter days are ahead."
            ],
            suggestions: vec![
                "Try listening to your favorite comforting music.",
                "Reach out to a friend or family member for support.",
                "Practice self-care with a warm bath or your favorite treat."
            ]
        }),
        ("anxious", MoodResponse {
            messages: vec![
                "Anxiety can be overwhelming, but you're not alone in this.",
                "Take deep breaths. You're stronger than you think.",
                "It's okay to feel anxious. Let's work through this together."
            ],
            suggestions: vec![
                "Try a 5-minute meditation or breathing exercise.",
                "Write down your worries and then set them aside.",
                "Go for a gentle walk to clear your mind."
            ]
        }),
        ("excited", MoodResponse {
            messages: vec![
                "Your excitement is palpable! What's got you so thrilled?",
                "Excitement is such a wonderful energy to have!",
                "Channel that excitement into something productive and fun."
            ],
            suggestions: vec![
                "Plan out how to make the most of this exciting time.",
                "Share your excitement with someone who will celebrate with you.",
                "Use this energy to tackle a task you've been putting off."
            ]
        }),
        ("tired", MoodResponse {
            messages: vec![
                "Rest is important. Give yourself permission to recharge.",
                "Being tired is your body's way of asking for care.",
                "Take it easy today. You deserve some downtime."
            ],
            suggestions: vec![
                "Take a short nap if possible, or just rest your eyes.",
                "Prepare a healthy, energizing meal or snack.",
                "Create a calming bedtime routine for tonight."
            ]
        }),
        ("angry", MoodResponse {
            messages: vec![
                "Anger is a valid emotion. Let's find healthy ways to express it.",
                "It's okay to feel angry. What can we do to help you feel better?",
                "Strong emotions like anger can teach us about our boundaries."
            ],
            suggestions: vec![
                "Try physical activity like punching a pillow or going for a run.",
                "Write down what's making you angry and why.",
                "Practice deep breathing to help calm the intensity."
            ]
        }),
        ("grateful", MoodResponse {
            messages: vec![
                "Gratitude is such a powerful emotion. Cherish this feeling.",
                "Being grateful opens our hearts to more positivity.",
                "Your gratitude mindset is inspiring!"
            ],
            suggestions: vec![
                "Write down three things you're grateful for today.",
                "Express your gratitude to someone who helped you.",
                "Start a gratitude journal to continue this practice."
            ]
        })
    ];

    // Display available moods
    for (i, (mood_name, _)) in moods.iter().enumerate() {
        println!("{}. {}", i + 1, capitalize_first(mood_name));
    }

    println!();

    // Get user input
    let selected_mood = loop {
        println!("Enter the number corresponding to your mood:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input.parse::<usize>() {
            Ok(num) if num >= 1 && num <= moods.len() => {
                break &moods[num - 1];
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and {}", moods.len());
            }
        }
    };

    // Provide response
    let (mood_name, response) = selected_mood;
    let mut rng = rand::thread_rng();

    let message = response.messages.choose(&mut rng).unwrap();
    let suggestion = response.suggestions.choose(&mut rng).unwrap();

    println!();
    println!("Based on your mood '{}':", capitalize_first(mood_name));
    println!("💙 {}", message);
    println!("💡 Suggestion: {}", suggestion);
    println!();
    println!("Remember, your feelings are valid, and taking care of your emotional well-being is important.");
    println!("Come back anytime for more support!");
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}