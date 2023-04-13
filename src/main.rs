//src/input_file/uspop.csv
//src/output_file/output.csv

extern crate csv;

use std::io;

use std::fs::File;
use std::vec::Vec;
use std::fs;
use csv::ReaderBuilder;
use csv::Writer;

fn main() {
    clear_screen();
    print!("\n
    #########################################################################################
    #                           LAMB ESSENTIAL DATA PROCESSING                              #
    #########################################################################################
    by Gabriel Cachadiña
    
    ");

    print!("\n
    Select the operation mode:

    (1) input.csv contains data in chuncks of n data

        - input.csv:
        ax0,ax1,...,axn,ay0,ay1,...,ayn,az0,az1,...azn

    In this case input->1, in the next step you will be asked for the n size of the data


    (2) Test


    ");

    clear_screen();

    let mode: u32 = input_data();


    if mode == 1{
        println!("Mode 1 selected, please enter the number of columns for each acceleration (n):");

        let n = 1;
        mode1(n);

    }
    else{
        println!("no mode selected");
        std::process::abort(); //Closes the file
    }
}


fn mode1(n:u32){
    let file = File::open("src/input_file/input.csv").expect("Failed to open file");    //Loads the file
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);         //Reader builder

   

    let output_filename = "src/output_file/output.csv";                                 //Defines output file
    // Remove the output file if it exists
    if let Err(e) = fs::remove_file(output_filename) {
        println!("Failed to remove {}: {}", output_filename, e);
    }
    let mut writer = Writer::from_path(output_filename).unwrap();

    let mut data: Vec<Vec<f64>> = Vec::new();                                           //Data saved in a matrix


    // Get the number of rows and columns in the CSV file
    let mut nrows = 0;

    for result in reader.records() {                                                    //Go thought all records
        let record = result.expect("Failed to read record");
        let mut row: Vec<f64> = Vec::new();                                             //Creates a new row to store the new rows

        nrows += 1;                                                                     //Count the number of rows of the file

        for field in record.iter() {                                                    //iterates all the fields in a specific row
            let val = field.parse::<f64>().expect("Failed to parse f64 value");
            row.push(val);
        }
        data.push(row);                                                                 //Save the data in the vector data
    }

    println!("{}",nrows);


    let mut mean_x;
    let mut mean_y;
    let mut mean_z;

    let mut var_x;
    let mut var_y;
    let mut var_z;

    let mut sma;

    let mut ai;

    
    writer.write_record(&["mean_x","mean_y","mean_z","var_x","var_y","var_z","sma","ai"]).unwrap();


    for i in 0..nrows { 

        //-------------------------------------------------------
        //                  Mean
        //-------------------------------------------------------


        // Calculate the mean of the 1st bucket 
        mean_x = 0.0;
        for j in 0..300{
            mean_x = &data[i][j] + mean_x;
        }
        mean_x = mean_x/300.0;

        // Calculate the mean of the 2nd bucket 
        mean_y = 0.0;
        for j in 300..600{
            mean_y = &data[i][j] + mean_y;
        }
        mean_y = mean_y/300.0;


        // Calculate the mean of the 3rd bucket 
        mean_z = 0.0;
        for j in 600..900{
            mean_z = &data[i][j] + mean_z;
        }
        mean_z = mean_z/300.0;

        //-------------------------------------------------------
        //                  Variation
        //-------------------------------------------------------

        // Calculate the variation of the 1st bucket 
        var_x = 0.0;
        for j in 0..300{
            var_x = ((&data[i][j] - mean_x)*(&data[i][j] - mean_x))+var_x;
        }
        var_x = var_x/300.0;



        // Calculate the variation of the 2nd bucket 
        var_y = 0.0;
        for j in 300..600{
            var_y = ((&data[i][j] - mean_y)*(&data[i][j] - mean_y))+var_y;
        }
        var_y = var_y/300.0;

        // Calculate the variation of the 3rd bucket 
        var_z = 0.0;
        for j in 300..600{
            var_z = ((&data[i][j] - mean_z)*(&data[i][j] - mean_z))+var_z;
        }
        var_z = var_z/300.0;


        //-------------------------------------------------------
        //          Signal Magnitude Area (SMA)
        //-------------------------------------------------------

        sma = mean_x.abs() + mean_y.abs() + mean_z.abs();



        //-------------------------------------------------------
        //          Average intensity (AI)
        //-------------------------------------------------------
        ai = 0.0;
        for j in 0..300{
            ai = f64::sqrt((&data[i][j] * &data[i][j]) + (&data[i][j+300] * &data[i][j+300]) + (&data[i][j+600] * &data[i][j+600])) + ai ;
        }
        ai = ai/300.0;

        writer.write_record(&[&mean_x.to_string(),&mean_y.to_string(),&mean_z.to_string(),&var_x.to_string(),&var_y.to_string(),&var_z.to_string(),&sma.to_string(),&ai.to_string()]).unwrap();
    }

    // Flush the writer to ensure all data is written to disk
    writer.flush().unwrap();

    println!("output.csv ready!")
}



//Clears screen
fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
}

//Takes data from the terminal
fn input_data()->u32{
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let input: u32 = input_line.trim().parse().expect("Input not an integer");
    return input;
}