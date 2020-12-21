mod constants;
mod init;
mod state;
mod camera;
mod paddle;

fn main() -> amethyst::Result<()> {
    let mut game = init::init()?;
    game.run();

    Ok(())
}
