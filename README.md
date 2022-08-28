# Daily country background

Program that lets you change your desktop background depending on which country has a national day. If no country has a national day that day a random country will be shown.
If multiple countries share the same day one of them is chosen at random.
Currently shows a picture of the flag of the country as well as information about the day. This is however easily editable to for example show prettier backgrounds for each country if you have the rights to such pictures.

The list of national days is from [Wikipedia](https://en.wikipedia.org/wiki/National_day).

## Usage instructions

### Windows

To use the program if you are on windows you only need to clone the repo and execute the country_backgrounds.exe script.
For the script to work you also need the pictures folder and the national_days.csv. 

To avoid having to manually run the script once a day to update the script you can use the windows task scheduler and set up the script to run once a day.
As the action for the task I would set `cmd.exe` as the program and `/c {path_to_script}` as the optional argument.

### Other platforms

For other platforms you will have to build the script yourself with Rust. Follow the instructions to [download rust](https://www.rust-lang.org/tools/install) for your platform and then run `cargo build --release`.
Then put the resulting binary in the same folder as the pictures folder and the national_days.csv. The script relies on the crate [Wallpaper](https://crates.io/crates/wallpaper) and can only set the wallpaper if the platform is supported by this crate.

## Changing the data

If you want to change any of the data simply edit the national_days.csv. Here you can remove, edit or add countries (or whatever else you want).
If you add an entry or edit something in the name column you must ensure that there is a corresponding picture in the pictures folder. You can of course also change the pictures that are displayed for each country.

## Further development

This was mainly meant as an exercise for me to test out how working with Rust was, especially when it came to using external crates.
Now that I have gotten everything to work on my machine I am happy to leave the project for now, but if you want to try the script and something doesn't work submit an issue and I will look at it.
