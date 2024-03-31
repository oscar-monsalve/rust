fn main() {
    let days : [&str; 12]= ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for i in days {
        let day_0 = "A partridge in a pear tree";
        let day_1 = "Two turtle doves, and";
        let day_2 = "Three french hens";
        let day_3 = "Four calling birds";
        let day_4 = "Five golden rings";
        let day_5 = "Six geese a-laying";
        let day_6 = "Seven swans a-swimming";
        let day_7 = "Eight maids a-milking";
        let day_8 = "Nine ladies dancing";
        let day_9 = "Ten lords a-leaping";
        let day_10 = "Eleven pipers piping";
        let day_11 = "Twelve drummers drumming";

        println!("On the {i} day of Christmas, my true love sent to me");

        if i == days[0] {
            println!("{day_0} \n");
        }
        else if i == days[1] {
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[2] {
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[3] {
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[4] {
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[5] {
            println!("{day_5}");
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[6] {
            println!("{day_6}");
            println!("{day_5}");
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[7] {
            println!("{day_7}");
            println!("{day_6}");
            println!("{day_5}");
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[8] {
            println!("{day_8}");
            println!("{day_7}");
            println!("{day_6}");
            println!("{day_5}");
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[9] {
            println!("{day_9}");
            println!("{day_8}");
            println!("{day_7}");
            println!("{day_6}");
            println!("{day_5}");
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[10] {
            println!("{day_10}");
            println!("{day_9}");
            println!("{day_8}");
            println!("{day_7}");
            println!("{day_6}");
            println!("{day_5}");
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }
        else if i == days[11] {
            println!("{day_11}");
            println!("{day_10}");
            println!("{day_9}");
            println!("{day_8}");
            println!("{day_7}");
            println!("{day_6}");
            println!("{day_5}");
            println!("{day_4}");
            println!("{day_3}");
            println!("{day_2}");
            println!("{day_1}");
            println!("{day_0} \n");
        }

    }
}
