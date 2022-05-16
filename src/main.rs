use std::env::args;

//function to calculate velocity and angle of projectile collision
//parameters are as follows: x and y position of projectile two, velocity and angle of projectile two,
//radius of projectiles and the corseness of the checking velocity and angle 
fn calculate_collison(x2: f64,y2: f64,v2: f64 ,o2: f64 , r:f64, v_increment:f64 , v_inital:f64) {
    let mut v1=v_inital+v_increment;//sets an inital testing velocity for the function

    //loop does not have to be labelled, but it does make this more neat,
    //labeling is more of an acceted pratice in rust than c++.
  'first:  loop { //infinite loop, will run until exit condition is acheived

        'second: for o1 in 1..180{//for loop that iterates over 180 degrees

            //calculates time, y positions and displacement
            let t=x2/(v1*(o1 as f64).to_radians().cos()-v2*o2.to_radians().cos());
            let yf1=v1*(o1 as f64).to_radians().sin()*t-4.905*t*t;
            let yf2=v2*o2.to_radians().sin()*t-4.905*t*t+y2;
            let ydisp=yf1-yf2;
            //println!("{v1} {o1} {yf1} {yf2} {ydisp} {t}");//left in case erronus calculation is found and debugging is needed

            if yf1 < 0.|| yf2 < 0. {//if y positions are less than zero, repeat
                continue 'second;
            }

            else if t < 0.{//if t is negitive, condition cannot exist, repeat
                continue 'second;
            }

            else if ydisp.abs()<=r {//passing condition
                println!("Passing velocity is {v1:.8} and passing angle is {o1}.\n");
                break 'first;
            }

            else if ydisp>0. && v_increment<v1/(10. as f64).powf(15.){//Velocity bit limit condition
            //have to define 10 as a f64 because rust wants no uncetainty with functions within types

            //runs second function that changes angle decrement
                calculate_coll_2(x2, y2, v2, o2, r, v1, (o1-1) as f64 , 0.1);
                break 'first;
            }

            else if ydisp>0. {//if over shot the projectile, decrease velocity increment and recurse the function
                //then exit function
                calculate_collison(x2,y2,v2,o2,r,v_increment/10.,v1-v_increment);
                break 'first;
            }
        }
        //if loop iterates through the 180 degress and does not hit, or overshoot add to projectile ones velocity
        v1+=v_increment;
    }
}

//function that decreases angle increment for more fine tuning.
fn calculate_coll_2 (x2:f64, y2:f64, v2:f64, o2: f64, r:f64, v1:f64, mut o1:f64, mut o_increment:f64 ) {
    //In functions, you can define values with transfered ownership as parameters as mutable within the function

    'first: loop{

        'second: for _o_count in 1..10{//the _ in front of o_count just tells the compiler it is supposed
            //to be a unused variable so it stops throwing a hissy fit with warns.
            o1 +=o_increment;
            let t=x2/(v1*o1.to_radians().cos()-v2*o2.to_radians().cos());
            let yf1=v1*o1.to_radians().sin()*t-4.905*t*t;
            let yf2=v2*o2.to_radians().sin()*t-4.905*t*t+y2;
            let ydisp=yf1-yf2;
            //println!(" {o1}, {o_count}, {o_increment}, {ydisp}"); //line from testing

            if yf1 < 0.|| yf2 < 0. {//if y positions are less than zero, repeat
                continue 'second;
            }

            else if t < 0.{//if t is negitive, condition cannot exist, repeat
                continue 'second;
            }

            else if ydisp.abs()<=r {//passing condition
                println!("Passing velocity is {v1:.8} and passing angle is {o1}.\n");
                break 'first;
            }

            else if o_increment<1./(10. as f64).powf(12.){//bit limit passing condition, if only due to bit limitations
                println!("Reached Uncertanty Limit:{ydisp:.10} discrepancy at velocity: {v1:} and angle: {o1:}.\n");
                break 'first;
            }

            else if ydisp>0. {//if over shot the projectile, decrease angle and exit for loop
                o1-=o_increment;
                break 'second;
            }
        }
        o_increment=o_increment/10.;
    }
}

fn main(){
    //variables for command line input arguments. Initally a string, will be converted to a float
    //with next set of varaibles
    let inx=args().nth(1).expect("\n No command line Arguments input. The command line arguments should be input in this form:\n
      cargo run -- xpos ypos velocity angle(degrees) projectile_radius\n ");
    let iny=args().nth(2).expect("No y cordnate for second projectile.");
    let inv=args().nth(3).expect("No velocity for second projectile.");
    let ino=args().nth(4).expect("No angle for second projectile");
    let inr=args().nth(5).expect("No radius for projectiles");
    //expect is like unwrap, except a personalized message to console can be input. I use this to tell user to use console
    //arguements as an input because within program user inputs is annoying without the read! macro from the text_io crate

    //uses parse function to convert to a f64
    let proj_x=inx.parse::<f64>().expect("Unable to parse x cord");
    let proj_y=iny.parse::<f64>().expect("Unable to parse Y cord");
    let proj_v=inv.parse::<f64>().expect("Unable to parse velocity");
    let proj_o=ino.parse::<f64>().expect("Unable to parse angle");
    let proj_r=inr.parse::<f64>().expect("Unable to parse radius");

    //println!("{proj_x},{proj_y},{proj_v},{proj_o},{proj_r}"); //left in from testing
    
    //inputs command line arguments into the function
    calculate_collison(proj_x, proj_y, proj_v, proj_o, proj_r, 10.0, 0.0);
}