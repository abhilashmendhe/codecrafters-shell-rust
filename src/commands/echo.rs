pub fn echo_cmd(trim_input: &str) {

    let mut next_str = &trim_input[5..];

    if let Some(ind) = next_str.chars().position(|x| x=='\'' || x=='\"' || x.is_alphanumeric() || x == '\\') {
        next_str = &next_str[ind..];
    } 

    let mut str = String::new();

    let mut space_flag = true;
    let mut single_oc_flag = false;
    let mut double_oc_flag = false;
    let mut slash_flag = false;

    for (_ind, ch) in next_str.char_indices() {

        if slash_flag {
            // println!("slahsh true for {} at index: {}",ch,ind);
            // println!("{}", ch=='\'');
            // println!("single - {}, double - {}", single_oc_flag, double_oc_flag);
            // println!("{}", str);
            if double_oc_flag {
                if ch == '\\' || ch == '"' {
                    str.push(ch);
                } else if ch == ' ' {
                    // println!("heere for space after \\");
                    str.push('\\');
                    str.push(' ');
                } else {
                    str.push('\\');
                    str.push(ch);
                }
            } else {
                str.push(ch);
            }
            // if ch == '\'' {
            //     str.push('\\');
            //     str.push(ch);

            // } else {
            //     // if ch == ' ' {
            //         if single_oc_flag || double_oc_flag {
            //             str.push('\\');
            //         } else {
            //             str.push(ch);
            //         }
            //     // } else {
            //     //     str.push(ch);
            //     // }
            // }

            slash_flag = false;
            continue;
        }
        if ch == '\\' && !single_oc_flag {
            slash_flag = true;
            continue;
        }   
        if ch == '\'' {
            if double_oc_flag {
                str.push('\'');
                continue;
            }
            if single_oc_flag {
                single_oc_flag = false;
            } else {
                single_oc_flag = true;
            }
            continue;
        } 

        if ch == '"' {
            if single_oc_flag {
                str.push('"');
                continue;
            }
            if double_oc_flag {
                double_oc_flag = false;
            } else {
                double_oc_flag = true;
            }
            continue;
        }

        if ch == ' ' {
            if single_oc_flag || double_oc_flag || space_flag {
                str.push(' ');
                // println!("ikde pro");
                // // if space_flag {
                // //     str.push(' ');
                    space_flag = false;
                // // }
            } 
            if space_flag {
                str.push(' ');
                space_flag = false;
            }

            continue;
        }
        str.push(ch);
        space_flag = true;
    }

    println!("{}", str);


}