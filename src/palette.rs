#[derive(Copy, Clone, Debug)]
pub struct Palette {
  pub hidden_colour: &'static str,
  pub standard_colour: &'static str,
  pub hidden_symlink_colour: &'static str,
  pub standard_symlink_colour: &'static str,
}

pub static DIRECTORIES_PALETTE : Palette = Palette {
  hidden_colour : &"\x1b[38;2;140;85;24m",
  standard_colour : &"\x1b[38;2;230;115;10m",
  hidden_symlink_colour : &"\x1b[38;2;170;60;230m",
  standard_symlink_colour : &"\x1b[38;2;220;90;255m",
};

pub static FILES_PALETTE : Palette = Palette {
  hidden_colour : &"\x1b[38;2;30;150;30m",
  standard_colour : &"\x1b[38;2;60;230;60m",
  // hidden_symlink_colour : &"\x1b[38;2;230;70;165m",
  // standard_symlink_colour : &"\x1b[38;2;245;120;180m",
  hidden_symlink_colour : &"\x1b[38;2;65;160;220m",
  standard_symlink_colour : &"\x1b[38;2;100;200;255m",
};
