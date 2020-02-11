use std::io;
fn main() {
    println!("Enter a word");
   let mut word = String::new();
   let mut word_new = String::new();
  // let mut final_word = String::new();
   io::stdin().read_line(&mut word)
                                .expect("failed to read line");
                                word = word.trim().to_string();
    let word_len = word.len();
   for i in (0..word_len+1).rev(){
   // println!("{}", i);
    if  i != 0{
   // print!("\n{}",&word[i-1..i] );
   word_new= word_new + &(&word[i-1..i]).to_string();
    }
   }
   //println!("{}",&word_new) ;
 

            // // Loop Again


            // for i in (0..word_len).rev(){
            //     // println!("{}", i);
            //      if  i!= 0 {
            //     // print!("\n{}",&word[i-1..i] );
            //     final_word= final_word + &(&word_new[i..i+1]).to_string();
            //      }
            //     }
      //          println!("{}",&final_word) ;

//println!("{}",&word[1..2] );

                // println!("{:?}",word);
                // println!("{:?}",word_new );
              //  println!("{}",final_word );



        if word == word_new {
           println!("{} is a palindrome", word)
        }else {
            println!("{} is a not palindrome", word)
            
        }
       

}
