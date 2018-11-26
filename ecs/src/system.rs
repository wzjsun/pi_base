
pub trait System: 'static + Sized {
	/**
	 * 运行
	 */
	fn run();
	/**
	 * 销毁
	 */
	fn destroy();
}

pub trait SystemMgr: 'static + Sized {
    fn new() -> Self;
}