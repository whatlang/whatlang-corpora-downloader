use whatlang::Lang;
use std::fs::File;
use std::fs;
use flate2::read::GzDecoder;
use tar::Archive;
use std::path::{Path, PathBuf};
use enum_map::enum_map;
use futures::future;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let sources = enum_map! {
        Lang::Epo => "epo_mixed_2012_100K",
        Lang::Eng => "eng_wikipedia_2016_100K",
        Lang::Rus => "rus_wikipedia_2016_100K",
        Lang::Cmn => "cmn_wikipedia_2016_100K",
        Lang::Spa => "spa_wikipedia_2016_100K",
        Lang::Por => "por_wikipedia_2016_100K",
        Lang::Ita => "ita_wikipedia_2016_100K",
        Lang::Ben => "ben_wikipedia_2016_30K",
        Lang::Fra => "fra_wikipedia_2010_100K",
        Lang::Deu => "deu_wikipedia_2016_100K",
        Lang::Ukr => "ukr_mixed_2012_100K",
        Lang::Kat => "kat_wikipedia_2016_100K",
        Lang::Ara => "ara_wikipedia_2016_100K",
        Lang::Hin => "hin_mixed_2019_100K",
        Lang::Jpn => "jpn_wikipedia_2016_100K",
        Lang::Heb => "heb_wikipedia_2016_100K",
        Lang::Yid => "yid_wikipedia_2016_30K",
        Lang::Pol => "pol_wikipedia_2016_100K",
        Lang::Amh => "amh_wikipedia_2016_30K",
        Lang::Jav => "jav_wikipedia_2016_100K",
        Lang::Kor => "kor_wikipedia_2016_100K",
        Lang::Nob => "nob_wikipedia_2014_100K",
        Lang::Nno => "nno_wikipedia_2016_100K",
        Lang::Dan => "dan_mixed_2014_100K",
        Lang::Swe => "swe_wikipedia_2016_100K",
        Lang::Fin => "fin_wikipedia_2016_100K",
        Lang::Tur => "tur_mixed_2014_100K",
        Lang::Nld => "nld_mixed_2012_100K",
        Lang::Hun => "hun_mixed_2012_100K",
        Lang::Ces => "ces_wikipedia_2016_100K",
        Lang::Ell => "ell_wikipedia_2016_100K",
        Lang::Bul => "bul_wikipedia_2016_100K",
        Lang::Bel => "bel_wikipedia_2016_100K",
        Lang::Mar => "mar_wikipedia_2016_100K",
        Lang::Kan => "kan_wikipedia_2016_100K",
        Lang::Ron => "ron_wikipedia_2018_100K",
        Lang::Slv => "slv_wikipedia_2016_100K",
        Lang::Hrv => "hrv_wikipedia_2016_100K",
        Lang::Srp => "srp_wikipedia_2016_100K",
        Lang::Mkd => "mkd_wikipedia_2016_100K",
        Lang::Lit => "lit_wikipedia_2016_100K",
        Lang::Lav => "lav_wikipedia_2014_100K",
        Lang::Est => "est_newscrawl_2017_100K",
        Lang::Tam => "tam_wikipedia_2016_100K",
        Lang::Vie => "vie_mixed_2014_100K",
        Lang::Urd => "urd_wikipedia_2016_100K",
        Lang::Tha => "tha_newscrawl_2011_100K",
        Lang::Guj => "guj_wikipedia_2016_100K",
        Lang::Uzb => "uzb_wikipedia_2016_100K",
        Lang::Pan => "pan_wikipedia_2016_100K",
        Lang::Aze => "aze_wikipedia_2016_100K",
        Lang::Ind => "ind_mixed_2012_100K",
        Lang::Tel => "tel_wikipedia_2016_100K",
        Lang::Pes => "pes_wikipedia_2016_100K",
        Lang::Mal => "mal_wikipedia_2016_100K",
        Lang::Ori => "ori_wikipedia_2016_30K",
        Lang::Mya => "", // has own script
        Lang::Bho => "", // See https://omniglot.com/language/phrases/bhojpuri.php
        Lang::Tgl => "tgl_wikipedia_2016_100K",
        Lang::Yor => "yor_wikipedia_2016_10K",
        Lang::Mai => "",  // see https://mai.wikipedia.org/wiki/%E0%A4%AE%E0%A5%88%E0%A4%A5%E0%A4%BF%E0%A4%B2%E0%A5%80_%E0%A4%AD%E0%A4%BE%E0%A4%B7%E0%A4%BE
        Lang::Orm => "",  // see https://om.wikipedia.org/wiki/Afaan_Oromoo
        Lang::Ibo => "",  // see https://ig.wikipedia.org/wiki/As%E1%BB%A5%CC%80s%E1%BB%A5%CC%80_%C3%8Cgb%C3%B2
        Lang::Ceb => "ceb_wikipedia_2016_100K",
        Lang::Kur => "kur_newscrawl_2011_30K",
        Lang::Mlg => "mlg_wikipedia_2014_30K",
        Lang::Skr => "",  // see https://skr.wikipedia.org/wiki/%D8%B3%D8%B1%D8%A7%D8%A6%DB%8C%DA%A9%DB%8C_%D8%B2%D8%A8%D8%A7%D9%86
        Lang::Nep => "nep_wikipedia_2016_100K",
        Lang::Sin => "sin_wikipedia_2016_100K",
        Lang::Khm => "", // https://km.wikipedia.org/wiki/%E1%9E%97%E1%9E%B6%E1%9E%9F%E1%9E%B6%E1%9E%81%E1%9F%92%E1%9E%98%E1%9F%82%E1%9E%9A
        Lang::Tuk => "tuk_wikipedia_2016_30K",
        Lang::Som => "som_newscrawl_2011_100K",
        Lang::Aka => "aka_wikipedia_2018",
        Lang::Zul => "zul_mixed_2014_100K",
        Lang::Kin => "kin_community_2017_30K",
        Lang::Hat => "hat-ht_web_2015_30K",
        Lang::Ilo => "ilo_wikipedia_2016_10K",
        Lang::Run => "",  // https://rn.wikipedia.org/wiki/Ikirundi
        Lang::Sna => "sna-zw_web_2018_100K",
        Lang::Uig => "uig_wikipedia_2016_30K",
        Lang::Afr => "afr_mixed_2019_100K",
        Lang::Lat => "lat_wikipedia_2018_100K",
        Lang::Slk => "slk_wikipedia_2016_100K",
        Lang::Cat => "cat_wikipedia_2016_100K",
    };


    let target_dir: PathBuf = Path::new(file!())
        .canonicalize()?
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .join("whatlang-corpora/corpora");

    println!("TARGET = {}\n", target_dir.to_string_lossy());

    for (lang, slug) in &sources {
        println!("{:?} {:?}", lang, slug);
        if !slug.is_empty() {
            download_corpus(lang, slug, &target_dir).await?;
        }
    }

    Ok(())
}

async fn download_corpus(lang: Lang, slug: &str, target_dir: &PathBuf) -> Result<()> {
    println!("Start download {}", lang.code());
    let sentences = obtain_sentences(slug).await?;
    let text = sentences.join("\n");
    let code = lang.code();
    let dest = target_dir.join(format!("{}.txt", code));
    std::fs::write(&dest, text)?;
    println!("Finished {}", lang.code());
    Ok(())
}

fn corpus_url(slug: &str) -> String {
    format!("https://pcai056.informatik.uni-leipzig.de/downloads/corpora/{}.tar.gz", slug)
}

async fn obtain_sentences(slug: &str) -> Result<Vec<String>> {
    let url = corpus_url(slug);
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let bytes: &[u8] = bytes.as_ref();
    let tar = GzDecoder::new(bytes);
    let mut archive = Archive::new(tar);
    let entries = archive.entries().expect(&format!("Failed to obtain entries from: {}", url));
    let mut sentences_entry = find_sentences_txt_entry(entries);

    let tmp_dir = std::env::temp_dir();
    let path = tmp_dir.join("whatlang-corpora-tmp.txt");
    sentences_entry.unpack(&path).expect("Failed to unpack");

    let mut sentences = vec![];
    let tmpfile: std::fs::File = File::open(&path).expect("failed to open a tmp file");
    use std::io::prelude::*;
    let buf_reader = std::io::BufReader::new(tmpfile);
    for line_res in buf_reader.lines() {
        let line = line_res.expect("Failed to get a line");
        // Get rid of number and obtain the sentence
        let sentence = line.split('\t').nth(1).unwrap().to_string();
        sentences.push(sentence);
    }

    Ok(sentences)
}

fn find_sentences_txt_entry<'a, 'b>(entries: tar::Entries<'a, GzDecoder<&'b [u8]>>) -> tar::Entry<'a, GzDecoder<&'b [u8]>> {
    for entry_result in entries {
        match entry_result {
            Ok(entry) => {
                let path = entry.path().expect("Can not get entry.path()");
                let path: &str = path.as_ref().to_str().unwrap();
                if path.contains("sentences.txt") {
                    return entry;
                }
            },
            Err(err) => {
                eprintln!("Could not read entry from tar::Entries: {}", err);
            }
        }
    }
    panic!("File sentences.txt is not found")
}

async fn find_slugs() {
    for lang in Lang::values() {
        let slug = find_slug_for(lang).await.unwrap_or("NOT_FOUND".into());
        println!("Lang::{:?} => \"{}\",", lang, slug);
    }
}

async fn find_slug_for(lang: Lang) -> Option<String> {
    for source in &["mixed", "wikipedia", "newscrawl"] {
        for year in (2010i32..=2021).rev() {
            for size in &["100K", "30K"] {
                let slug = format!("{}_{}_{}_{}", lang.code(), source, year, size);
                if probe_slug(&slug).await {
                    return Some(slug);
                }
            }
        }
    }
    None
}

async fn probe_slug(slug: &str) -> bool {
    let url = corpus_url(slug);
    let response = reqwest::get(&url).await.expect("Failed to probe slug");
    response.status().is_success()
}
