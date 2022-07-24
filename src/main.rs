use rs_logger::*;


fn main() {
    let mut logger = Logger::default();
    logger.critical("%D %T gdfs");
    logger.error("%T fdsgdfs");
    logger.debug("%D fdsgfds");
}   
