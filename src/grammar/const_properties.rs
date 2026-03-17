pub const PROPERTY1: &str =
r#"Program
+-> always = always
    +-> -> = ->
        +-> = = =
        |   +-> % = %
        |   |   +-> TIME = t
        |   |   +-> TIMEUNIT = h
        |   |       +-> NUMBER = 24
        |   +-> NUMBER = 0
        +-> < = <
            +-> always = always
            |   +-> Interval
            |   |   +-> TIMEUNIT = h
            |   |   |   +-> NUMBER = 0
            |   |   +-> TIMEUNIT = h
            |   |       +-> NUMBER = 24
            |   +-> sumtime = sumtime
            |       +-> * = *
            |           +-> active = active
            |           +-> power = power
            +-> POWERUNIT = kwh
                +-> NUMBER = 10
"#;

pub const PROPERTY2: &str =
r#"Program
+-> eventually = eventually
    +-> ! = !
    +-> > = >
        +-> count = count
        |   +-> active = active
        +-> NUMBER = 5
"#;

pub const PROPERTY3: &str =
r#"Program
+-> always = always
    +-> foreach = foreach
        +-> -> = ->
            +-> active = active
            +-> eventually = eventually
                +-> Interval
                |   +-> TIMEUNIT = h
                |   |   +-> NUMBER = 0
                |   +-> TIMEUNIT = h
                |       +-> NUMBER = 6
                +-> ! = !
                    +-> active = active
"#;

pub const PROPERTY4: &str =
r#"Program
+-> always = always
    +-> count = count
        +-> & = &
            +-> = = =
            |   +-> name = name
            |   +-> STRING = fridge
            +-> active = active
"#;

pub const PROPERTY5: &str =
r#"Program
+-> always = always
    +-> -> = ->
        +-> >= = >=
        |   +-> count = count
        |   |   +-> active = active
        |   +-> NUMBER = 5
        +-> < = <
            +-> eventually = eventually
            |   +-> Interval
            |   |   +-> TIMEUNIT = h
            |   |   |   +-> NUMBER = 0
            |   |   +-> TIMEUNIT = h
            |   |       +-> NUMBER = 6
            |   +-> count = count
            |       +-> active = active
            +-> NUMBER = 5
"#;

pub const PROPERTY6: &str =
r#"Program
+-> always = always
    +-> <= = <=
        +-> sum = sum
        |   +-> * = *
        |       +-> active = active
        |       +-> power = power
        +-> POWERUNIT = w
            +-> NUMBER = 100
"#;

pub const PROPERTY7: &str =
r#"Program
+-> always = always
|   +-> NUMBER = 7
+-> eventually = eventually
    +-> NUMBER = 7
"#;

