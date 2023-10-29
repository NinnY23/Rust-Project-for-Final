use std::{io, vec};
use std::error::Error;
use std::path::Path;
use csv::Writer;

#[derive(Debug)]
struct Vector{
    x: f64,
    y: f64,
    z: f64 
}

impl Vector {
    fn new(x:f64, y:f64, z:f64) -> Self {
        Vector{x,y,z}
    }
    fn add(&self, other: &Vector) -> Vector{
        Vector::new(self.x+other.x, self.y + other.y, self.z + other.z)
    }
    fn substract(&self, other: &Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    fn dot_product(&self, other: &Vector) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z 
    }
    fn cross_product(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.x * other.z - self.z * other.x,
            self.x * other.y - self.y * other.x,
        )
    }
    fn display(&self) {
        println!("Vector: ({}, {}, {})", self.x, self.y, self.z);
    }
}

fn read_vector() -> Vector {
    println!("Enter x y z coordinates (space-separated):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let coordinates: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    if coordinates.len() != 3 {
        panic!("Invalid number of coordinates provided");
    }

    Vector::new(coordinates[0], coordinates[1], coordinates[2])
}

fn vector_operations() -> Result<(), Box<dyn Error>>{
    println!("Enter your first vector values: ");
    let vector1 = read_vector();
    println!("Enter your second vector values: ");
    let vector2 = read_vector();
    let result1 = vector1.add(&vector2);
    let result2 = vector1.substract(&vector2);
    let result3 = vector1.dot_product(&vector2);
    let result4 = vector1.cross_product(&vector2);
    println!("\nVector Addition:\n{:?}", result1);
    println!("\nVector Substraction:\n{:?}", result2);
    println!("\nDot Product:\n{:?}", result3);
    println!("\nCross Product:\n{:?}", result4);

    let file_path = "vector_operations.csv";
    let mut writer = Writer::from_path(file_path)?;

    writer.write_record(&["Vector Operation", "Result"])?;
    writer.write_record(&["Vector Addition", &format!("[{:?} {:?} {:?}]", result1.x, result1.y, result1.z)])?;
    writer.write_record(&["Vector Subtraction", &format!("[{:?} {:?} {:?}]", result2.x, result2.y, result2.z)])?;
    writer.write_record(&["Dot Product", &format!("{}", result3)])?;
    writer.write_record(&["Cross Product", &format!("[{:?} {:?} {:?}]", result4.x, result4.y, result4.z)])?;

    println!("Results have been written to {}", file_path);

    Ok(())
}

#[derive(Debug)]
struct Matrix{
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>
}

impl Matrix{
    fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        Matrix{ rows, cols, data }
    }

    fn add(&self, other: &Matrix) -> Matrix{
        let mut result_data = vec![vec![0.0; self.cols]; self.rows]; 
        for i in 0..self.rows{
            for j in 0..self.cols{
                result_data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Matrix::new(self.rows, other.cols, result_data)
    }

    fn subtract(&self, other: &Matrix) -> Matrix {
        let mut result_data = vec![vec![0.0; self.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result_data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Matrix::new(self.rows, self.cols, result_data)
    }


    fn multiplication(&self, other: &Matrix) -> Matrix {
        let mut result_data = vec![vec![0.0; other.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result_data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix::new(self.rows, other.cols, result_data)
    }

    fn transpose(&self) -> Matrix {
        let mut result_data = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows{
            for j in 0..self.cols{
                result_data[j][i] = self.data[i][j];
            }
        }
        Matrix::new(self.cols, self.rows, result_data)
    }

    fn display_matrix(&self) {
        println!("Matrix:");
        for row in &self.data {
            println!("{:?}", row);
        }
    }
}

fn read_matrix() -> Matrix {
    println!("Enter number of rows:");
    let mut rows = String::new();
    io::stdin().read_line(&mut rows).expect("Failed to read line");
    let rows: usize = rows.trim().parse().expect("Invalid input");

    println!("Enter number of columns:");
    let mut cols = String::new();
    io::stdin().read_line(&mut cols).expect("Failed to read line");
    let cols: usize = cols.trim().parse().expect("Invalid input");

    println!("Enter matrix elements (row-wise, space-separated):");
    let mut data = Vec::new();
    for _ in 0..rows {
        let mut row_input = String::new();
        io::stdin().read_line(&mut row_input).expect("Failed to read line");
        let row_elements: Vec<f64> = row_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        data.push(row_elements);
    }

    Matrix { rows, cols, data }
}

fn matrix_operations() -> Result<(), Box<dyn Error>>{
    println!("Enter Matrix 1:");
    let matrix1 = read_matrix();
    println!("Enter Matrix 2:");
    let matrix2 = read_matrix();

    matrix1.display_matrix();
    
    matrix2.display_matrix();

    let result1 = matrix1.add(&matrix2).data;
    println!("\nMatix Addition: ");
    for row in &result1 {
        let formatted_row: Vec<String> = row.iter().map(|&x| format!("{:>4.1}", x)).collect();
        println!("[{}]", formatted_row.join(", "));
    }
    
    let result2 = matrix1.subtract(&matrix2).data;
    println!("\nMatix Subtraction: ");
    for row in &result2 {
        let formatted_row: Vec<String> = row.iter().map(|&x| format!("{:>4.1}", x)).collect();
        println!("[{}]", formatted_row.join(", "));
    }
    
    let result3 = matrix1.multiplication(&matrix2).data;
    println!("\nMatrix Multiplication: ");
    for row in &result3 {
        let formatted_row: Vec<String> = row.iter().map(|&x| format!("{:>4.1}", x)).collect();
        println!("[{}]", formatted_row.join(", "));
    }

    let result4 = matrix1.transpose().data;
    println!("\nTransposed of Matrix 1:");
    for row in &result4 {
        let formatted_row: Vec<String> = row.iter().map(|&x| format!("{:>4.1}", x)).collect();
        println!("[{}]", formatted_row.join(", "));
    }

    let result5 = matrix2.transpose().data;
    println!("\nTransposed of Matrix 2:");
    for row in &result5 {
        let formatted_row: Vec<String> = row.iter().map(|&x| format!("{:>4.1}", x)).collect();
        println!("[{}]", formatted_row.join(", "));
    }

    let file_path = "matrix_operations.csv";
    let mut writer = Writer::from_path(file_path)?;

    writer.write_record(&["Matrix Operation", "Result"])?;
    writer.write_record(&["Matrix Addition", &format!("{:?}", result1)])?;
    writer.write_record(&["Matrix Subtraction", &format!("{:?}", result2)])?;
    writer.write_record(&["Matrix Multiplication", &format!("{:?}", result3)])?;
    writer.write_record(&["Transposed Matrix 1", &format!("{:?}", result4)])?;
    writer.write_record(&["Transposed Matrix 2", &format!("{:?}", result5)])?;

    println!("Results saved to {}", file_path);
    Ok(())
}

#[allow(unused_must_use)]
#[derive(Debug)]
#[derive(Clone)]
struct Set {
    elements: Vec<i32>,
}

#[allow(unused_must_use)]
impl Set {
    fn new() -> Self {
        Set { elements: Vec::new() }
    }

    fn add(&mut self, element: i32) {
        if !self.elements.contains(&element) {
            self.elements.push(element);
        }
    }

    fn remove(&mut self, element: i32) {
        if let Some(index) = self.elements.iter().position(|&x| x == element) {
            self.elements.remove(index);
        }
    }

    fn empty_set(&self) -> bool {
        self.elements.is_empty()
    }

    fn equal_sets(&self, other: &Set) -> bool {
        self.elements.len() == other.elements.len() && self.elements.iter().all(|e| other.elements.contains(e))
    }

    fn unequal_sets(&self, other: &Set) -> bool {
        !self.equal_sets(other)
    }

    fn equivalent_sets(&self, other: &Set) -> bool {
        self.equal_sets(other)
    }

    fn overlapping_sets(&self, other: &Set) -> bool {
        self.elements.iter().any(|e| other.elements.contains(e))
    }

    fn disjoint_sets(&self, other: &Set) -> bool {
        !self.overlapping_sets(other)
    }

    // Check if this set is a subset of another set
    fn is_subset(&self, other: &Set) -> bool {
        self.elements.iter().all(|e| other.elements.contains(e))
    }

    // Check if this set is a superset of another set
    fn is_superset(&self, other: &Set) -> bool {
        other.is_subset(self)
    }

    // Calculate the power set of this set
    fn power_set(&self) -> Vec<Vec<i32>> {
        let mut power_set: Vec<Vec<i32>> = Vec::new();
        let set_size = self.elements.len();
        let total_subsets = 1 << set_size;

        for i in 0..total_subsets {
            let mut subset: Vec<i32> = Vec::new();
            for j in 0..set_size {
                if (i >> j) & 1 == 1 {
                    subset.push(self.elements[j]);
                }
            }
            power_set.push(subset);
        }

        power_set
    }
}

fn read_set() -> Set {
    println!("Enter elements of the set (space-separated integers):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let elements: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    Set { elements }
}

#[allow(unused_must_use)]
fn set_operations() -> Result<(), Box<dyn Error>>{
    let set1 = read_set();
    let set2 = read_set();

    println!("\n--------------Checking Sets----------------\n");

    println!("\nSet 1: \n{:?}", set1.elements);
    println!("\nSet 2: \n{:?}", set2.elements);

    println!("\nIs Set 1 empty? \n{}", set1.empty_set());
    println!("\nIs Set 2 empty? \n{}", set2.empty_set());

    println!("\nIs it equal Sets? \n{}", set1.equal_sets(&set2));
    println!("\nIs it unequal Sets? \n{}", set1.unequal_sets(&set2));
    println!("\nIs it equivalent Sets? \n{}", set1.equivalent_sets(&set2));
    println!("\nIs it overlapping Sets? \n{}", set1.overlapping_sets(&set2));
    println!("\nIs it disjoint Sets? \n{}", set1.disjoint_sets(&set2));

    println!("\nSet 1 is Subset of Set 2? \n{}", set1.is_subset(&set2));
    println!("\nSet 1 is Superset of Set 2? \n{}", set1.is_superset(&set2));

    let power_set1 = set1.power_set();
    let power_set2 = set2.power_set();

    println!("\nPower Set of Set 1: \n{:?}", power_set1);
    println!("\nPower Set of Set 2: \n{:?}", power_set2);

    println!("-------------Adding and Removing Sets--------------");

    println!("Enter new set to add or remove the element:");
    let mut set = read_set();

    // Add elements to the set
    println!("Enter element to add:");
    let mut add_element = String::new();
    io::stdin().read_line(&mut add_element).expect("Failed to read line");
    let add_element: i32 = add_element.trim().parse().expect("Invalid input");
    set.add(add_element);
    println!("Set after adding {}: {:?}", add_element, set.elements);

    // Remove an element from the set
    println!("Enter element to remove:");
    let mut remove_element = String::new();
    io::stdin().read_line(&mut remove_element).expect("Failed to read line");
    let remove_element: i32 = remove_element.trim().parse().expect("Invalid input");
    set.remove(remove_element);
    println!("Set after removing {}: {:?}", remove_element, set.elements);

    // Open the CSV file for writing
    let file_path = "set_operations.csv";
    let mut writer = Writer::from_path(file_path)?;

    // Write set operation results to the CSV file
    writer.write_record(&["Is Empty Set", &set1.empty_set().to_string()]);
    writer.write_record(&["Is Equal Sets", &set1.equal_sets(&set2).to_string()]);
    writer.write_record(&["Is Unequal Sets", &set1.unequal_sets(&set2).to_string()]);
    writer.write_record(&["Is Equivalent Sets", &set1.equivalent_sets(&set2).to_string()]);
    writer.write_record(&["Is Overlapping Sets", &set1.overlapping_sets(&set2).to_string()]);
    writer.write_record(&["Is Disjoint Sets", &set1.disjoint_sets(&set2).to_string()]);
    writer.write_record(&["Is Subset", &set1.is_subset(&set2).to_string()]);
    writer.write_record(&["Is Superset", &set1.is_superset(&set2).to_string()]);

    println!("Results saved to {}", file_path);
    // Close the CSV writer
    writer.flush();

    Ok(())
}


#[derive(Debug)]
struct BooleanLogic {
    a: bool,
    b: bool, 
}

impl BooleanLogic {
    fn new(a: bool, b: bool) -> Self {
        BooleanLogic { a, b }
    }

    fn logical_and(&self) -> bool {
        self.a && self.b
    }

    fn logical_or(&self) -> bool {
        self.a || self.b
    }

    fn logical_not_a(&self) -> bool {
        !self.a
    }

    fn logical_not_b(&self) -> bool {
        !self.b
    }

}

fn read_booleanlogic() -> BooleanLogic {
    println!("Enter value for 'a' (true or false):");
    let a: bool = read_boolean_input();
    println!("Enter value for 'b' (true or false):");
    let b: bool = read_boolean_input();

    BooleanLogic::new(a, b)
}

fn read_boolean_input() -> bool {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "true" => return true,
            "false" => return false,
            _ => {
                println!("Invalid input. Please enter 'true' or 'false'.");
                continue;
            }
        }
    }
}

fn booleanlogic_operations() -> Result<(), Box<dyn Error>>{
    let bl = read_booleanlogic();

    println!("Logical AND (a && b): {}", bl.logical_and());
    println!("Logical OR (a || b): {}", bl.logical_or());
    println!("Logical NOT for a (!a): {}", bl.logical_not_a());
    println!("Logical NOT for b (!b): {}", bl.logical_not_b());

    let file_path = "BooleanLogic_Operation.csv";
    let mut writer = Writer::from_path(file_path)?;

    writer.write_record(&["Logical AND (a && b)", &bl.logical_and().to_string()])?;
    writer.write_record(&["Logical OR (a || b)", &bl.logical_or().to_string()])?;
    writer.write_record(&["Logical NOT for a (!a)", &bl.logical_not_a().to_string()])?;
    writer.write_record(&["Logical NOT for b (!b)", &bl.logical_not_b().to_string()])?;

    writer.flush()?;
    Ok(())
}


#[derive(Debug, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    fn add(&self, other: &Complex) -> Complex {
        Complex { 
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    fn subtract(&self, other: &Complex) -> Complex {
        Complex { 
            real: self.real - other.real, 
            imag: self.imag - other.imag,
        }
    }

    fn multiply(&self, other: &Complex) -> Complex {
        Complex { 
            real: self.real * other.real - self.imag * other.imag, 
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

fn read_complex() -> Complex {
    println!("Enter real part:");
    let real: f64 = read_f64_input();
    println!("Enter imaginary part:");
    let imag: f64 = read_f64_input();

    Complex::new(real, imag)
}

fn read_f64_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a valid floating-point number.");
                continue;
            }
        }
    }
}

fn complex_operations() -> Result<(), Box<dyn Error>>{
    println!("Enter first complex: ");
    let complex1 = read_complex();

    println!("Enter second complex: ");
    let complex2 = read_complex();
    
    let sum = complex1.add(&complex2);
    println!("\nSum: \nReal:{:.2}\nImaginary:{:.2}", sum.real, sum.imag);

    let difference = complex1.subtract(&complex2);
    println!("\nDifference: \nReal:{:.2}\nImaginary:{:.2}", difference.real, difference.imag);

    let product = complex1.multiply(&complex2);
    println!("\nProduct: \nReal:{:.2}\nImaginary:{:.2}", product.real, product.imag);

    let file_path = "Complex_Operations.csv";
    let mut writer = Writer::from_path(file_path)?;

    writer.write_record(&["Operation", "Real", "Imaginary"])?;
    writer.write_record(&["Sum", &sum.real.to_string(), &sum.imag.to_string()])?;
    writer.write_record(&["Difference", &difference.real.to_string(), &difference.imag.to_string()])?;
    writer.write_record(&["Product", &product.real.to_string(), &product.imag.to_string()])?;

    writer.flush()?;
    println!("CSV output written successfully to 'Complex_Operations.csv'.");
    Ok(())
}
#[allow(unused_must_use)]

fn main() -> Result<(), Box<dyn Error>>{
    loop {
        println!("Select your options: ");
        println!("1. Calculate Vector.");
        println!("2. Calculate Matrix.");
        println!("3. Calculate Set.");
        println!("4. Calculate Boolean Logic");
        println!("5. Calculate Complex");
        println!("6. Return to the main menu");
        println!("7. Exit Program.");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice); // user input
        let choice = choice.trim();

    match choice {
        "1" => vector_operations(),
        "2" => matrix_operations(),
        "3" => set_operations(),
        "4" => booleanlogic_operations(),
        "5" => complex_operations(),
        "6" => continue,
        "7" => {
            println!("Exit the program. Goodbye!");
            break Ok(());
        }
        _ => {
            println!("Invalid choice. Please try again.");
            Err("Invalid choice".into()) // Return an Err variant with an error message
        }
    };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(4.0, 5.0, 6.0);

        let result_addition = vector1.add(&vector2);
        assert_eq!(result_addition.x, 5.0);
        assert_eq!(result_addition.y, 7.0);
        assert_eq!(result_addition.z, 9.0);

        let result_subtraction = vector1.substract(&vector2);
        assert_eq!(result_subtraction.x, -3.0);
        assert_eq!(result_subtraction.y, -3.0);
        assert_eq!(result_subtraction.z, -3.0);

        let result_dot_product = vector1.dot_product(&vector2);
        assert_eq!(result_dot_product, 32.0);

        let result_cross_product = vector1.cross_product(&vector2);
        assert_eq!(result_cross_product.x, -3.0);
        assert_eq!(result_cross_product.y, -6.0);
        assert_eq!(result_cross_product.z, -3.0);
    }

    #[test]
    fn test_matrix_operations () {
        let matrix1 = Matrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let matrix2 = Matrix::new(2, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

        let result1 = matrix1.add(&matrix2);
        assert_eq!(result1.data, vec![vec![6.0, 8.0], vec![10.0, 12.0]]);

        let result2 = matrix1.subtract(&matrix2);
        assert_eq!(result2.data, vec![vec![-4.0, -4.0], vec![-4.0, -4.0]]);

        let result3 = matrix1.multiplication(&matrix2);
        assert_eq!(result3.data, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);

        let result4 = matrix1.transpose();
        assert_eq!(result4.data, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);

        let result = matrix2.transpose();
        assert_eq!(result.data, vec![vec![5.0, 7.0], vec![6.0, 8.0]]);
    }

    #[test]
    fn test_set_operations() {
        let mut set1 = Set::new();
        set1.add(1);
        set1.add(2);

        let mut set2 = Set::new();
        set2.add(2);
        set2.add(3);

        assert_eq!(set1.empty_set(), false);
        assert_eq!(set2.empty_set(), false);

        assert_eq!(set1.equal_sets(&set2), false);
        assert_eq!(set1.unequal_sets(&set2), true);
        assert_eq!(set1.equivalent_sets(&set2), false);
        assert_eq!(set1.overlapping_sets(&set2), true);
        assert_eq!(set1.disjoint_sets(&set2), false);

        assert_eq!(set1.is_subset(&set2), false);
        assert_eq!(set1.is_superset(&set2), false);

        let power_set1 = set1.power_set();
        let power_set2 = set2.power_set();

        assert_eq!(power_set1, vec![vec![], vec![1], vec![2], vec![1, 2]]);
        assert_eq!(power_set2, vec![vec![], vec![2], vec![3], vec![2, 3]]);
    }

    #[test]
    fn test_boolean_logic_operations() {
        let bl = BooleanLogic::new(true, false);

        assert_eq!(bl.logical_and(), false);
        assert_eq!(bl.logical_or(), true);
        assert_eq!(bl.logical_not_a(), false);
        assert_eq!(bl.logical_not_b(), true);
    }

    #[test]
    fn test_complex_operations() {
        let complex1 = Complex::new(1.0, 2.0);
        let complex2 = Complex::new(3.0, 4.0);

        let result_addition = complex1.add(&complex2);
        assert_eq!(result_addition, Complex::new(4.0, 6.0));

        let result_subtraction = complex1.subtract(&complex2);
        assert_eq!(result_subtraction, Complex::new(-2.0, -2.0));

        let result_multiplication = complex1.multiply(&complex2);
        assert_eq!(result_multiplication, Complex::new(-5.0, 10.0));
    }
}