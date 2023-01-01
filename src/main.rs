use std::fs;



fn main() {
    struct Node {
        id: i32,
        lat: f64,
        lon: f64,
    }
    let file_name = "./dj38.tsp";

    println!("Reading the file: {}", file_name);

    let file_content = fs::read_to_string(file_name)
        .expect("Failed to read the file");

    let mut nodes = vec![];

    for line in file_content.split("\n"){
        if line.is_empty(){
          //if the line is empty, nothing should be done   
        }
        else if line.chars().last().unwrap() == '0' {
            //println!("{}", line); //printing for debug

            let mut position = 1;
            let mut id = -1;
            let mut lat = -1.0;
            let mut lon = -1.0;

            for word in line.split(' '){
                if position == 1{
                    id = word.parse::<i32>().unwrap();
                }
                else if position == 2{
                    lat = word.parse::<f64>().unwrap();
                }
                else if position == 3{
                    lon = word.parse::<f64>().unwrap();
                }
                position = position+1;
            }
            let node_i = Node{id: id,lat: lat,lon: lon};
            nodes.push(node_i);
        }
    }//end of reading nodes

    for node in nodes{
        println!("{}",node.id)
    }

    // Clark and Wright algoritm: - applied as at https://web.mit.edu/urban_or_book/www/book/chapter6/6.4.12.html

    //Step 1: Calculate the savings for every pair

    //depot is node 0

    fn distance(node_1:Node,node_2:Node) {
        f64::powf(
            f64::powf((node_1.lat - node_2.lat), 2.0) +
            f64::powf((node_1.lon - node_2.lon),2.0),0.5);      
    }

    //saving[i,j] = distance(D, i) + distance(D, j) - distance(i, j)

    
}
