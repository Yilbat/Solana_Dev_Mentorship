// Grade Calculator program 

use std::io;

fn main() {
    // Step 1: Declare variables for the name and score
    // Using user input for both name and score
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // Removing trailing newline

    println!("Enter your score (0-100):");
    let mut score_input = String::new();
    io::stdin().read_line(&mut score_input).expect("Failed to read line");
    let score: f32 = score_input.trim().parse().expect("Please enter a valid number");

    // Step 2: Call the function to calculate the grade
    let grade = calculate_grade(score);

    // Step 3: Use control flow to print a custom message based on the grade
    if grade == "A" {
        println!("Excellent work, {}!", name);
    } else if grade == "B" {
        println!("Good job, {}! ", name);
    } else if grade == "C" {
        println!("You passed, {}. Keep improving!", name);
    } else if grade == "D" {
        println!("{}: Try harder next time!", name);
    } else if grade == "F" {
        println!("{}: Don't give up! Keep studying!", name);
    } else {
        println!("{}: Invalid score entered!", name);
    }

    // Step 4: Print the final score and grade
    println!("Score: {}, Grade: {}", score, grade);
}

// Step 5: Write a function to calculate the grade based on the score
fn calculate_grade(score: f32) -> &'static str {
    if score < 0.0 || score > 100.0 {
        return "Invalid"; // Handling edge case for invalid score
    } else if score >= 90.0 {
        "A"
    } else if score >= 80.0 {
        "B"
    } else if score >= 70.0 {
        "C"
    } else if score >= 60.0 {
        "D"
    } else {
        "F"
    }
}


// Replace hardcoded values with user input and modify the code above by:


// 1. Adding more detailed messages for each grade.


// 2. Handling edge cases, like invalid scores or negative numbers.

