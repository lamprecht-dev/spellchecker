use std::fs;
use std::cmp;

#[derive(Debug)]
struct WordReport{
    word: String,
    correct: bool,
    improvements: Option<Vec<String>>    
}

fn main() {
    // So this is very slow... Here are ways I want to imrpove the speed in future!
    // Step 1. Instead of a list of all words I want to store it into a word tree.
    // This has the benefit that sea and see are stored both under se and thus automatically have a distance of 1 to their parent AND EACH OTHER!
    // Now we can traverse through the tree with our wagner_fisher but also use pruning to get rid of whole branches we know will be slower than our best case!!

    let result = spellcheck("Hello thjis is mmy random text. Please give me all mispelled words and ther potential coerrections".to_string());

    dbg!(result.iter().filter(|w| !w.correct).collect::<Vec<_>>());
}

fn load_all_words() -> Vec<String>{
    return fs::read_to_string("./src/words.txt")
        .expect("Should have been able to read file")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
}

fn wagner_fisher(s: &str, t: &str) -> usize{
    // Input is two words, output is the distance from one word to another
    // for all i and j, d[i,j] will hold the distance between
    // the first i characters of s and the first j characters of t
    // note that d has (m+1)*(n+1) values
    let mut d:Vec<Vec<usize>> = Vec::new();

    for i in 0..=s.len(){
        let mut dd:Vec<usize> = Vec::new();
        for j in 0..=t.len(){
            if i == 0{
                dd.push(j);
            }
            else if j == 0{
                dd.push(i);
            }
            else{
                dd.push(0);
            }
        } 
        d.push(dd);
    }

    for j in 1..=t.len(){
        for i in 1..=s.len(){
            let change_cost = if s[i-1..i] == t[j-1..j] { 0 } else { 1 };

            d[i][j] = *[
                d[i - 1][j] + 1, //delete
                d[i][j - 1] + 1, //insert
                d[i - 1][j - 1] + change_cost //substitute
            ].iter().min().unwrap();
        }
    }

    d[s.len()][t.len()]
}

fn spellcheck(text: String) -> Vec<WordReport>{
    let all_words = load_all_words();
    let given_words = text.split(" ").collect::<Vec<_>>();

    given_words
    .iter()
    .map(|w| {
        // Exists in list
        if all_words.contains(&w.to_lowercase().to_string()){
            return WordReport{word: w.to_string(), correct: true, improvements: None};
        }

        //Still need to deal with words that contain special characters. Or punctuation.



        let mut shortest = usize::MAX;
        let thershold = 0; //0 means we only take the words that are closest to being correct without any regards for bigger spelling mistakes that make the word look like something else.
        let distances = all_words.iter().map(|aw|{
            let dist = wagner_fisher(aw, w); 
            shortest = cmp::min(shortest, dist);
            (dist, aw)
        }).collect::<Vec<_>>();
        
        let valid = distances.iter().filter_map(|(d, w)|{
            if *d <= shortest + thershold{
                return Some(w.to_string());
            } 
            None
        }).collect::<Vec<_>>();
        
        WordReport{word: w.to_string(), correct: false, improvements: Some(valid)}
    }).collect::<Vec<WordReport>>()        
}