pub struct CatFrames {
    current_frame: usize,
    frames: Vec<String>,
}

impl Iterator for CatFrames {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_frame >= self.frames.len() {
            self.current_frame = 0;
        }

        let frame = self.frames[self.current_frame].clone();
        self.current_frame += 1;

        Some(frame)
    }
}

pub fn cat() -> CatFrames {
    CatFrames {
        current_frame: 0,
        frames: vec![
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
         \\
          V"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
          \\
           V"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
          \\
            >"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔\\▔▔\
          \__>"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔\\▔▔\
          \__>"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
          \\
            >"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
         \\
          V"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
         ||
         v"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
        //
        V"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
        //
       <"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔//▔▔\
      <__/"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔//▔▔\
      <__/"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
        //
       <"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
        //
        V"#,
            r#"    /\_/\
   >(•༝• )< 
    |    \
    || |  )
    ll-l-// 
/▔▔▔▔▔▔▔▔||▔▔\
         ||
         v"#,
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    }
}
