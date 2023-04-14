//src/input_file/uspop.csv
//src/output_file/output.csv

extern crate csv;

use std::io;

use std::fs::File;
use std::vec::Vec;
use std::fs;
use csv::ReaderBuilder;
use csv::Writer;


struct Matrix{
    pub rows:usize,
    pub cols:usize,
    pub values:Vec<Vec<f64>>
}


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
        print!("\
        Mode 1 selected, do you want the resulting data to be scalated (from 0 to 1)?
        (1) Yes
        (0) No
        ");
        let n = input_data();
        let values = mode1();

        if n==1{
            scale(values);
        }

    }
    else{
        println!("no mode selected");
        std::process::abort(); //Closes the file
    }
}






fn mode1()->Matrix{
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

    let mut values : Vec<Vec<f64>> = vec![vec![0.0;8];nrows]; //Data saved as a 2d vector
    
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
        values[i][0] = mean_x;

        // Calculate the mean of the 2nd bucket 
        mean_y = 0.0;
        for j in 300..600{
            mean_y = &data[i][j] + mean_y;
        }
        mean_y = mean_y/300.0;
        values[i][1] = mean_y;


        // Calculate the mean of the 3rd bucket 
        mean_z = 0.0;
        for j in 600..900{
            mean_z = &data[i][j] + mean_z;
        }
        mean_z = mean_z/300.0;
        values[i][2] = mean_z;

        //-------------------------------------------------------
        //                  Variation
        //-------------------------------------------------------

        // Calculate the variation of the 1st bucket 
        var_x = 0.0;
        for j in 0..300{
            var_x = ((&data[i][j] - mean_x)*(&data[i][j] - mean_x))+var_x;
        }
        var_x = var_x/300.0;
        values[i][3] = var_x;



        // Calculate the variation of the 2nd bucket 
        var_y = 0.0;
        for j in 300..600{
            var_y = ((&data[i][j] - mean_y)*(&data[i][j] - mean_y))+var_y;
        }
        var_y = var_y/300.0;
        values[i][4] = var_y;


        // Calculate the variation of the 3rd bucket 
        var_z = 0.0;
        for j in 300..600{
            var_z = ((&data[i][j] - mean_z)*(&data[i][j] - mean_z))+var_z;
        }
        var_z = var_z/300.0;
        values[i][5] = var_z;


        //-------------------------------------------------------
        //          Signal Magnitude Area (SMA)
        //-------------------------------------------------------

        sma = mean_x.abs() + mean_y.abs() + mean_z.abs();
        values[i][6] = sma;


        //-------------------------------------------------------
        //          Average intensity (AI)
        //-------------------------------------------------------
        ai = 0.0;
        for j in 0..300{
            ai = f64::sqrt((&data[i][j] * &data[i][j]) + (&data[i][j+300] * &data[i][j+300]) + (&data[i][j+600] * &data[i][j+600])) + ai ;
        }
        ai = ai/300.0;
        values[i][7] = ai;

 
        //println!("{},{},{},{},{},{},{},{}",values[i][0],values[i][1],values[i][2],values[i][3],values[i][4],values[i][5],values[i][6],values[i][7]);
        writer.write_record(&[&mean_x.to_string(),&mean_y.to_string(),&mean_z.to_string(),&var_x.to_string(),&var_y.to_string(),&var_z.to_string(),&sma.to_string(),&ai.to_string()]).unwrap();
    }

    // Flush the writer to ensure all data is written to disk
    writer.flush().unwrap();

    println!("output.csv ready!");

    let output = Matrix{
        rows: nrows,
        cols: 7,
        values: values,
    };

    return output;
}


fn scale(input:Matrix){
    let mut x_mean_min:f64 = input.values[0][0];
    let mut x_mean_max:f64 = input.values[0][0];
    let mut y_mean_min:f64 = input.values[0][1];
    let mut y_mean_max:f64 = input.values[0][1];
    let mut z_mean_min:f64 = input.values[0][2];
    let mut z_mean_max:f64 = input.values[0][2];

    let mut x_var_min:f64 = input.values[0][3];
    let mut x_var_max:f64 = input.values[0][3];
    let mut y_var_min:f64 = input.values[0][4];
    let mut y_var_max:f64 = input.values[0][4];
    let mut z_var_min:f64 = input.values[0][5];
    let mut z_var_max:f64 = input.values[0][5];


    let mut sma_min:f64 = input.values[0][6];
    let mut sma_max:f64 = input.values[0][6];


    let mut ai_min:f64 = input.values[0][7];
    let mut ai_max:f64 = input.values[0][7];

    //Obtain the min and max values for normalizing
    for i in 0..input.rows{

        // Means
        if input.values[i][0] < x_mean_min {
            x_mean_min = input.values[i][0];
        }
        if input.values[i][0] > x_mean_max {
            x_mean_max = input.values[i][0];
        }


        if input.values[i][1] < y_mean_min {
            y_mean_min = input.values[i][1];
        }
        if input.values[i][1] > y_mean_max {
            y_mean_max = input.values[i][1];
        }


        if input.values[i][2] < z_mean_min {
            z_mean_min = input.values[i][2];
        }
        if input.values[i][2] > z_mean_max {
            z_mean_max = input.values[i][2];
        }




        // Variations
        if input.values[i][3] < x_var_min {
            x_var_min = input.values[i][3];
        }
        if input.values[i][3] > x_var_max {
            x_var_max = input.values[i][3];
        }


        if input.values[i][4] < y_var_min {
            y_var_min = input.values[i][4];
        }
        if input.values[i][4] > y_var_max {
            y_var_max = input.values[i][4];
        }


        if input.values[i][5] < z_var_min {
            z_var_min = input.values[i][5];
        }
        if input.values[i][5] > z_var_max {
            z_var_max = input.values[i][5];
        }

        //sma
        if input.values[i][6] < sma_min {
            sma_min = input.values[i][6];
        }
        if input.values[i][6] > sma_max {
            sma_max = input.values[i][6];
        }

        //ai
        if input.values[i][7] < ai_min {
            ai_min = input.values[i][7];
        }
        if input.values[i][7] > ai_max {
            ai_max = input.values[i][7];
        }
    }

    let max_min = [[x_mean_min,x_mean_max],[y_mean_min,y_mean_max],[z_mean_min,z_mean_max],[x_var_min,x_var_max],[y_var_min,y_var_max],[z_var_min,z_var_max],[sma_min,sma_max],[ai_min,ai_max]];
    //Knowing the maximun and minimun values we proceed to normalize the data and save it in output.csv


    //println!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",x_mean_min,x_mean_max,y_mean_min,y_mean_max,z_mean_min,z_mean_max,x_var_min,x_var_max,y_var_min,y_var_max,z_var_min,z_var_max,sma_min,sma_max,ai_min,ai_max);
    //println!("{:?}",max_min);

    let mut output = Matrix{
        rows: input.rows,
        cols: input.cols,
        values: input.values,
    };


    let output_filename = "src/output_file/output_scaled.csv";                                 //Defines output file
    // Remove the output file if it exists
    if let Err(e) = fs::remove_file(output_filename) {
        println!("Failed to remove {}: {}", output_filename, e);
    }
    let mut writer = Writer::from_path(output_filename).unwrap();
    
    writer.write_record(&["mean_x","mean_y","mean_z","var_x","var_y","var_z","sma","ai"]).unwrap();

    for i in 0..output.rows{
        for j in 0..output.cols+1{
            output.values[i][j] = (output.values[i][j] - max_min[j][0])/(max_min[j][1]- max_min[j][0]);
        }
        writer.write_record(&[&output.values[i][0].to_string(),&output.values[i][1].to_string(),&output.values[i][2].to_string(),&output.values[i][3].to_string(),&output.values[i][4].to_string(),&output.values[i][5].to_string(),&output.values[i][6].to_string(),&output.values[i][7].to_string()]).unwrap();
    }

    //print!("{:?}",output.values);

    writer.flush().unwrap();

    println!("output.csv ready!");

}




//Clears screen
fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
}

//Takes u32 from the terminal
fn input_data()->u32{
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let input: u32 = input_line.trim().parse().expect("Input not an integer");
    return input;
}