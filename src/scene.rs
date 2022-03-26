
use crate::context::GMContext;
use crate::error::GMError;

pub trait GMScene {
    fn enter(&mut self, _context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }
    fn update_before(&mut self, _context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }
    fn update_after(&mut self, _context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }
    fn draw_before(&mut self, _context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }
    fn draw_after(&mut self, _context: &mut GMContext)  -> Result<(), GMError> {
        Ok(())
    }
    fn leave(&mut self, _context: &mut GMContext) -> Result<(), GMError> {
        Ok(())
    }
}
