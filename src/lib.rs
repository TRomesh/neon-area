use neon::prelude::*;

struct Area {
    pub pi: f64,
}

impl Area {
    fn circle(&self, radius: f64) -> f64 {
        return 2.00 * self.pi * radius;
    }
    fn squar(&self, length: f64, width: f64) -> f64 {
        return length * width;
    }

    fn traingle(&self, length: f64, width: f64, height: f64) -> f64 {
        let s: f64 = (length + width + height) / 2.0;
        let area: f64 = s * (s - length) * (s - width) * (s - height);
        return area.sqrt();
    }

    fn pentagon(&self, length: f64) -> f64 {
        return ((f64::sqrt(5.00 * (5.00 + (2.00 * f64::sqrt(5.00))))) / 4.00) * (length * length);
    }

    fn hexagon(&self, length: f64) -> f64 {
        return (3.00 * f64::sqrt(3.00)) / 2.00 * length;
    }

    fn trapezium(&self, length: f64, base: f64, height: f64) -> f64 {
        return (length * base) / 2.00 * height;
    }
}

fn area_of_circle(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let radius = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let area = Area { pi: 3.14 };
    Ok(cx.number(area.circle(radius)))
}

fn area_of_squar(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let width = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let length = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let area = Area { pi: 3.14 };
    Ok(cx.number(area.squar(width, length)))
}

fn area_of_traingle(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let width = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let length = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let height = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let area = Area { pi: 3.14 };
    Ok(cx.number(area.traingle(width, length, height)))
}

fn area_of_hexagon(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let length = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let area = Area { pi: 3.14 };
    Ok(cx.number(area.hexagon(length)))
}

fn area_of_pentagon(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let length = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let area = Area { pi: 0.00 };
    Ok(cx.number(area.pentagon(length)))
}

fn area_of_trapezium(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let length = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let base = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let height = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let area = Area { pi: 0.00 };
    Ok(cx.number(area.trapezium(length, base, height)))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("area_of_circle", area_of_circle)?;
    cx.export_function("area_of_squar", area_of_squar)?;
    cx.export_function("area_of_traingle", area_of_traingle)?;
    cx.export_function("area_of_hexagon", area_of_hexagon)?;
    cx.export_function("area_of_pentagon", area_of_pentagon)?;
    cx.export_function("area_of_trapezium", area_of_trapezium)?;
    Ok(())
}
