use super::{Request, Response};

pub trait Middle {
    fn before(req: Request);
    fn after(res: Response);
}
