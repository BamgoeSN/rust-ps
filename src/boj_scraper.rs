use scraper::Html;
use std::rc::Rc;

/// Returns a vector of pairs of sample inputs and outputs.
pub fn get_boj_samples(id: u32) -> impl Iterator<Item = (String, String)> {
	let doc = Rc::new(get_boj_problem(id));
	let h1 = doc.clone();
	boj_sample_inputs(h1).zip(boj_sample_outputs(doc))
}

fn get_boj_problem(id: u32) -> Html {
	let url = format!("https://www.acmicpc.net/problem/{id}");
	get_html(&url)
}

fn boj_sample_inputs(doc: Rc<Html>) -> impl Iterator<Item = String> {
	(1..).map_while(move |id| {
		let query = scraper::Selector::parse(&format!("#sample-input-{id}")).unwrap();
		doc.select(&query).map(|x| x.inner_html()).next()
	})
}

fn boj_sample_outputs(doc: Rc<Html>) -> impl Iterator<Item = String> {
	(1..).map_while(move |id| {
		let query = scraper::Selector::parse(&format!("#sample-output-{id}")).unwrap();
		doc.select(&query).map(|x| x.inner_html()).next()
	})
}

fn get_html(url: &str) -> Html {
	let request = reqwest::blocking::get(url).unwrap().text().unwrap();
	let parsed = Html::parse_document(&request);
	parsed
}
