/* 
In this exercise you're going to write some code to help you cook a brilliant lasagna from your favorite cooking book.

You have four tasks, all related to the time spent cooking the lasagna.

1. Define the expected oven time in minutes
Define the expected_minutes_in_oven binding to check how many minutes the lasagna should be in the oven. According to the cooking book, the expected oven time in minutes is 40:
expected_minutes_in_oven()
// Returns: 40

2. Calculate the remaining oven time in minutes
Define the remaining_minutes_in_oven function that takes the actual minutes the lasagna has been in the oven as a parameter and returns how many minutes the lasagna still has to remain in the oven, based on the expected oven time in minutes from the previous task.
remaining_minutes_in_oven(30)
// Returns: 10

3. Calculate the preparation time in minutes
Define the preparation_time_in_minutes function that takes the number of layers you added to the lasagna as a parameter and returns how many minutes you spent preparing the lasagna, assuming each layer takes you 2 minutes to prepare.
preparation_time_in_minutes(2)
// Returns: 4

4. Calculate the elapsed time in minutes
Define the elapsed_time_in_minutes function that takes two parameters: the first parameter is the number of layers you added to the lasagna, and the second parameter is the number of minutes the lasagna has been in the oven. The function should return how many minutes you've worked on cooking the lasagna, which is the sum of the preparation time in minutes, and the time in minutes the lasagna has spent in the oven at the moment.
elapsed_time_in_minutes(3, 20)
// Returns: 26 
*/


//SOLUTION

// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    let  expected_minutes_in_oven = 40;
     expected_minutes_in_oven
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
        expected_minutes_in_oven() - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
        number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
        actual_minutes_in_oven + preparation_time_in_minutes (number_of_layers)
}


//Explanation

// Most of the code is simple & pretty much self explanatory (basic maths). You can refer Q&A