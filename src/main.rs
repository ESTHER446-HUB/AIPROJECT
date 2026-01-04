use rand::seq::SliceRandom;
use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct MoodResponse {
    messages: Vec<&'static str>,
    suggestion_categories: HashMap<&'static str, Vec<&'static str>>,
}

fn get_suggestion_categories_for_mood(mood: &str) -> Vec<&'static str> {
    match mood {
        "happy" => vec!["Social Connection", "Self-Reflection", "Nature & Movement", "Creative Expression", "Celebration"],
        "sad" => vec!["Comfort Activities", "Social Support", "Self-Care", "Creative Outlets", "Mindful Observation"],
        "anxious" => vec!["Breathing Exercises", "Grounding Techniques", "Physical Movement", "Emotional Processing", "Support Seeking"],
        "excited" => vec!["Planning & Organization", "Sharing & Celebration", "Productive Action", "Creative Visualization", "Energy Expression"],
        "tired" => vec!["Rest & Recovery", "Nutrition & Energy", "Sleep Routine", "Gentle Movement", "Digital Detox"],
        "angry" => vec!["Physical Release", "Emotional Processing", "Communication", "Creative Expression", "Self-Regulation"],
        "grateful" => vec!["Reflection Practice", "Expression of Thanks", "Journaling", "Sharing Gratitude", "Mindful Appreciation"],
        "stressed" => vec!["Stress Reduction", "Task Management", "Environmental Changes", "Boundary Setting", "Relaxation Techniques"],
        "lonely" => vec!["Social Connection", "Community Engagement", "Self-Compassion", "Creative Activities", "Pet Companionship"],
        "motivated" => vec!["Goal Setting", "Productivity Boost", "Reward Systems", "Accountability", "Action Planning"],
        _ => vec!["General Support"],
    }
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
                "Your happiness lights up the world around you.",
                "What a beautiful day to be feeling so good!",
                "Your positive energy is truly inspiring."
            ],
            suggestion_categories: [
                ("Social Connection", vec![
                    "Share your joy with a loved one by calling them.",
                    "Plan a fun outing with friends or family.",
                    "Send a positive message to someone you care about."
                ]),
                ("Self-Reflection", vec![
                    "Take a moment to journal what made you happy today.",
                    "Reflect on your accomplishments and strengths.",
                    "Write down three things you're looking forward to."
                ]),
                ("Nature & Movement", vec![
                    "Go for a walk in nature to amplify your good mood.",
                    "Try a new outdoor activity like hiking or biking.",
                    "Dance to your favorite music in your living room."
                ]),
                ("Creative Expression", vec![
                    "Try a new hobby or activity you've been curious about.",
                    "Create something artistic like drawing or crafting.",
                    "Express your happiness through music or singing."
                ]),
                ("Celebration", vec![
                    "Celebrate your happiness by treating yourself to something special.",
                    "Plan a small reward for maintaining your positive outlook.",
                    "Share your good mood by doing something kind for others."
                ]),
            ].iter().cloned().collect(),
        }),
        ("sad", MoodResponse {
            messages: vec![
                "I'm here for you. It's okay to feel sad sometimes.",
                "Sadness is a normal emotion. Give yourself time to process it.",
                "Remember that this feeling will pass, and brighter days are ahead.",
                "Your feelings are completely valid. Be gentle with yourself.",
                "Even on sad days, you're still worthy of love and care."
            ],
            suggestion_categories: [
                ("Comfort Activities", vec![
                    "Try listening to your favorite comforting music.",
                    "Wrap yourself in a cozy blanket and relax.",
                    "Have a warm drink like tea or hot chocolate."
                ]),
                ("Social Support", vec![
                    "Reach out to a friend or family member for support.",
                    "Call someone who always makes you feel better.",
                    "Share your feelings with a trusted person."
                ]),
                ("Self-Care", vec![
                    "Practice self-care with a warm bath or your favorite treat.",
                    "Take extra time for yourself today.",
                    "Do something nurturing like reading or gentle stretching."
                ]),
                ("Creative Outlets", vec![
                    "Watch a feel-good movie or read an uplifting book.",
                    "Draw, paint, or create something to express your feelings.",
                    "Listen to music and let yourself feel the emotions."
                ]),
                ("Mindful Observation", vec![
                    "Take a quiet walk and notice the small beauties around you.",
                    "Spend time in nature observing the world around you.",
                    "Practice mindfulness by focusing on your breath."
                ]),
            ].iter().cloned().collect(),
        }),
        ("anxious", MoodResponse {
            messages: vec![
                "Anxiety can be overwhelming, but you're not alone in this.",
                "Take deep breaths. You're stronger than you think.",
                "It's okay to feel anxious. Let's work through this together.",
                "Your anxiety doesn't define you - you're capable and resilient.",
                "This feeling is temporary. You've gotten through tough moments before."
            ],
            suggestion_categories: [
                ("Breathing Exercises", vec![
                    "Try a 5-minute meditation or breathing exercise.",
                    "Practice the 4-7-8 breathing technique.",
                    "Use a breathing app for guided exercises."
                ]),
                ("Grounding Techniques", vec![
                    "Write down your worries and then set them aside.",
                    "Practice grounding by naming 5 things you can see, 4 you can touch, etc.",
                    "Hold an ice cube or splash cold water on your face."
                ]),
                ("Physical Movement", vec![
                    "Go for a gentle walk to clear your mind.",
                    "Try light stretching or yoga poses.",
                    "Dance to upbeat music to release tension."
                ]),
                ("Emotional Processing", vec![
                    "Journal about what's causing your anxiety.",
                    "Talk through your worries with a trusted friend.",
                    "Create a 'worry time' where you allow yourself to worry for 10 minutes only."
                ]),
                ("Support Seeking", vec![
                    "Call a trusted friend to talk through your feelings.",
                    "Consider speaking with a mental health professional.",
                    "Join an online support community for anxiety."
                ]),
            ].iter().cloned().collect(),
        }),
        ("excited", MoodResponse {
            messages: vec![
                "Your excitement is palpable! What's got you so thrilled?",
                "Excitement is such a wonderful energy to have!",
                "Channel that excitement into something productive and fun.",
                "This energy is perfect for starting something new!",
                "Your enthusiasm is contagious - keep that spark alive!"
            ],
            suggestion_categories: [
                ("Planning & Organization", vec![
                    "Plan out how to make the most of this exciting time.",
                    "Create a timeline for your exciting plans.",
                    "Make a list of steps to bring your excitement to life."
                ]),
                ("Sharing & Celebration", vec![
                    "Share your excitement with someone who will celebrate with you.",
                    "Tell friends and family about what's got you excited.",
                    "Celebrate your enthusiasm with a small treat."
                ]),
                ("Productive Action", vec![
                    "Use this energy to tackle a task you've been putting off.",
                    "Start working on that project you've been excited about.",
                    "Channel your excitement into learning something new."
                ]),
                ("Creative Visualization", vec![
                    "Create a vision board for your exciting plans.",
                    "Write down all the positive outcomes you imagine.",
                    "Spend time daydreaming about your goals."
                ]),
                ("Energy Expression", vec![
                    "Dance to your favorite upbeat music to celebrate.",
                    "Express your excitement through art or writing.",
                    "Share your energy by helping someone else with their goals."
                ]),
            ].iter().cloned().collect(),
        }),
        ("tired", MoodResponse {
            messages: vec![
                "Rest is important. Give yourself permission to recharge.",
                "Being tired is your body's way of asking for care.",
                "Take it easy today. You deserve some downtime.",
                "Listen to your body - it knows what it needs.",
                "Rest isn't lazy, it's necessary for your well-being."
            ],
            suggestion_categories: [
                ("Rest & Recovery", vec![
                    "Take a short nap if possible, or just rest your eyes.",
                    "Lie down for 20 minutes of quiet relaxation.",
                    "Give yourself permission to do nothing for a while."
                ]),
                ("Nutrition & Energy", vec![
                    "Prepare a healthy, energizing meal or snack.",
                    "Stay hydrated with water or herbal tea.",
                    "Eat foods that naturally boost energy levels."
                ]),
                ("Sleep Routine", vec![
                    "Create a calming bedtime routine for tonight.",
                    "Avoid screens for an hour before bed.",
                    "Make your sleep environment more comfortable."
                ]),
                ("Gentle Movement", vec![
                    "Try a gentle yoga session or stretching.",
                    "Take a slow, leisurely walk outside.",
                    "Do some light household tasks if you feel up to it."
                ]),
                ("Digital Detox", vec![
                    "Limit screen time and do something relaxing before bed.",
                    "Spend time away from your phone and computer.",
                    "Read a physical book or listen to calming music."
                ]),
            ].iter().cloned().collect(),
        }),
        ("angry", MoodResponse {
            messages: vec![
                "Anger is a valid emotion. Let's find healthy ways to express it.",
                "It's okay to feel angry. What can we do to help you feel better?",
                "Strong emotions like anger can teach us about our boundaries.",
                "Your anger is telling you something important about your needs.",
                "Channel that fire into positive change."
            ],
            suggestion_categories: [
                ("Physical Release", vec![
                    "Try physical activity like punching a pillow or going for a run.",
                    "Do some intense exercise to burn off the energy.",
                    "Scream into a pillow or tear up old newspapers."
                ]),
                ("Emotional Processing", vec![
                    "Write down what's making you angry and why.",
                    "Journal about the root cause of your anger.",
                    "Identify what boundaries have been crossed."
                ]),
                ("Communication", vec![
                    "Talk it out with someone you trust.",
                    "Practice assertive communication techniques.",
                    "Write a letter expressing your feelings (you don't have to send it)."
                ]),
                ("Creative Expression", vec![
                    "Create art or music to express your emotions.",
                    "Draw or paint how you're feeling.",
                    "Write poetry or lyrics about your anger."
                ]),
                ("Self-Regulation", vec![
                    "Practice deep breathing to help calm the intensity.",
                    "Use progressive muscle relaxation techniques.",
                    "Take a timeout to cool down before responding."
                ]),
            ].iter().cloned().collect(),
        }),
        ("grateful", MoodResponse {
            messages: vec![
                "Gratitude is such a powerful emotion. Cherish this feeling.",
                "Being grateful opens our hearts to more positivity.",
                "Your gratitude mindset is inspiring!",
                "What a wonderful way to view the world.",
                "Gratitude turns what we have into enough."
            ],
            suggestion_categories: [
                ("Reflection Practice", vec![
                    "Write down three things you're grateful for today.",
                    "Spend 5 minutes reflecting on positive aspects of your life.",
                    "Think about people who have supported you recently."
                ]),
                ("Expression of Thanks", vec![
                    "Express your gratitude to someone who helped you.",
                    "Send thank-you notes or messages to important people.",
                    "Verbally thank someone for their kindness."
                ]),
                ("Journaling", vec![
                    "Start a gratitude journal to continue this practice.",
                    "Write detailed entries about what you're thankful for.",
                    "Include photos or drawings in your gratitude journal."
                ]),
                ("Sharing Gratitude", vec![
                    "Share your gratitude with others around you.",
                    "Tell friends and family what you appreciate about them.",
                    "Post something positive on social media."
                ]),
                ("Mindful Appreciation", vec![
                    "Take a moment to appreciate the simple joys in life.",
                    "Practice mindful eating, savoring each bite.",
                    "Notice and appreciate small acts of kindness around you."
                ]),
            ].iter().cloned().collect(),
        }),
        ("stressed", MoodResponse {
            messages: vec![
                "Stress is common, but you don't have to carry it alone.",
                "Take a moment to breathe. You're doing your best.",
                "Let's find some ways to lighten your load.",
                "Your well-being matters. It's okay to prioritize yourself.",
                "Stress doesn't have to control your day."
            ],
            suggestion_categories: [
                ("Stress Reduction", vec![
                    "Try progressive muscle relaxation or deep breathing.",
                    "Listen to calming music or nature sounds.",
                    "Practice mindfulness meditation for 5-10 minutes."
                ]),
                ("Task Management", vec![
                    "Break down overwhelming tasks into smaller steps.",
                    "Prioritize your to-do list and focus on one thing at a time.",
                    "Delegate tasks if possible, or ask for help."
                ]),
                ("Environmental Changes", vec![
                    "Organize your space to create a sense of calm.",
                    "Declutter your immediate environment.",
                    "Create a peaceful, dedicated workspace."
                ]),
                ("Boundary Setting", vec![
                    "Practice saying 'no' to additional commitments.",
                    "Set clear boundaries with others about your time.",
                    "Learn to recognize when you're taking on too much."
                ]),
                ("Relaxation Techniques", vec![
                    "Take a short break to do something you enjoy.",
                    "Try aromatherapy with calming essential oils.",
                    "Spend time in nature or with calming scenery."
                ]),
            ].iter().cloned().collect(),
        }),
        ("lonely", MoodResponse {
            messages: vec![
                "Feeling lonely doesn't mean you're alone. I'm here with you.",
                "Connection is a basic human need. Be kind to yourself.",
                "Even in loneliness, you have value and worth.",
                "It's brave to acknowledge these feelings.",
                "You deserve connection and belonging."
            ],
            suggestion_categories: [
                ("Social Connection", vec![
                    "Reach out to an old friend for a quick chat.",
                    "Call a family member you haven't spoken to recently.",
                    "Send a message to someone you care about."
                ]),
                ("Community Engagement", vec![
                    "Join an online community related to your interests.",
                    "Volunteer or help someone else - it can reduce loneliness.",
                    "Participate in a local club or group activity."
                ]),
                ("Self-Compassion", vec![
                    "Practice self-kindness and remind yourself you're worthy.",
                    "Spend quality time alone doing something you enjoy.",
                    "Write a letter to yourself expressing compassion."
                ]),
                ("Creative Activities", vec![
                    "Try a new social activity or class.",
                    "Join a hobby group or creative workshop.",
                    "Explore online classes or virtual meetups."
                ]),
                ("Pet Companionship", vec![
                    "Spend quality time with a pet if you have one.",
                    "Consider volunteering at an animal shelter.",
                    "Watch cute animal videos for a mood boost."
                ]),
            ].iter().cloned().collect(),
        }),
        ("motivated", MoodResponse {
            messages: vec![
                "Your motivation is a powerful force! Use it wisely.",
                "This drive is exactly what you need right now.",
                "Channel that motivation into meaningful action.",
                "You're capable of amazing things when motivated.",
                "Keep this momentum going - you're on the right track!"
            ],
            suggestion_categories: [
                ("Goal Setting", vec![
                    "Set specific, achievable goals for today.",
                    "Write down your top 3 priorities for the day.",
                    "Create a timeline for your current project."
                ]),
                ("Productivity Boost", vec![
                    "Create a playlist of energizing music to maintain focus.",
                    "Set up your workspace for maximum productivity.",
                    "Use the Pomodoro technique for focused work sessions."
                ]),
                ("Reward Systems", vec![
                    "Reward yourself for completing tasks.",
                    "Plan small celebrations for reaching milestones.",
                    "Track your progress and acknowledge achievements."
                ]),
                ("Accountability", vec![
                    "Share your goals with an accountability partner.",
                    "Join a productivity group or challenge.",
                    "Check in with yourself daily on your progress."
                ]),
                ("Action Planning", vec![
                    "Break your big goals into daily actionable steps.",
                    "Create a detailed action plan for your motivation.",
                    "Identify potential obstacles and plan around them."
                ]),
            ].iter().cloned().collect(),
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

    // Get suggestion categories for this mood
    let categories = get_suggestion_categories_for_mood(mood_name);

    println!();
    println!("Great! For feeling {}, here are some types of support you can choose from:", capitalize_first(mood_name));
    println!();

    // Display categories
    for (i, category) in categories.iter().enumerate() {
        println!("{}. {}", i + 1, category);
    }

    println!();

    // Get user choice for category
    let selected_category = loop {
        println!("Enter the number for the type of support you'd like:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input.parse::<usize>() {
            Ok(num) if num >= 1 && num <= categories.len() => {
                break &categories[num - 1];
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and {}", categories.len());
            }
        }
    };

    // Get a random suggestion from the selected category
    let suggestion = response.suggestion_categories.get(*selected_category).unwrap().choose(&mut rng).unwrap();

    println!();
    println!("Based on your mood '{}' and choice of '{}':", capitalize_first(mood_name), selected_category);
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