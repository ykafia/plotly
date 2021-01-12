use crate::Layout;

/// A struct that implements `Trace` can be serialized to json format that is understood by Plotly.js.
pub trait Trace {
    fn serialize(&self) -> String;
}

/// Plot is a container for structs that implement the `Trace` trait. Optionally a `Layout` can
/// also be specified. Its function is to serialize `Trace`s and the `Layout` in html format and
/// display and/or persist the resulting plot.
///
/// # Examples
///
#[derive(Default)]
pub struct PlotData {
    traces: Vec<Box<dyn Trace>>,
    layout: Option<Layout>,
}
impl PlotData {
    /// Create a new `Plot`.
    pub fn new() -> PlotData {
        PlotData {
            traces: Vec::with_capacity(1),
            ..Default::default()
        }
    }

    /// Add a `Trace` to the `Plot`.
    pub fn add_trace(&mut self, trace: Box<dyn Trace>) {
        self.traces.push(trace);
    }

    /// Add multiple `Trace`s to the `Plot`.
    pub fn add_traces(&mut self, traces: Vec<Box<dyn Trace>>) {
        for trace in traces {
            self.add_trace(trace);
        }
    }

    /// Set the `Layout` to be used by `Plot`.
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
    }

    pub fn to_json(&self) -> String {
        let mut plot_data: Vec<String> = Vec::new();
        for trace in self.traces.iter() {
            let s = trace.serialize();
            plot_data.push(s);
        }
        let layout_data = match &self.layout {
            Some(layout) => Trace::serialize(layout),
            None => "{}".to_owned(),
        };

        let mut json_data = String::new();
        json_data.push_str(r#"{"data": ["#);

        for (index, data) in plot_data.iter().enumerate() {
            if index < plot_data.len() - 1 {
                json_data.push_str(data);
                json_data.push_str(r#","#);
            } else {
                json_data.push_str(data);
                json_data.push_str("]");
            }
        }
        json_data.push_str(format!(r#", "layout": {}"#, layout_data).as_str());
        json_data.push_str("}");
        json_data
    }
    pub fn to_inline_html(&self, id : &str) -> String {
        let template = r#"
            <div>
                <div id="[id]"/>
                <script>
                    var e = document.getElementById("[id]");
                    var json_plot = [json];
                    Plotly.newPlot( e, json_plot.data, json_plot.layout);
                </script>
            </div>
        "#;
        return template.replace("[id]",id).replace("[json]",self.to_json().as_str());
    }
    pub fn to_js_text(&self, id : &str) -> String {
        let template = r#"
            var e = document.getElementById("[id]");
            var json_plot = [json];
            Plotly.newPlot( e, json_plot.data, json_plot.layout);
        "#;
        return template.replace("[id]",id).replace("[json]",self.to_json().as_str());
    }
}
