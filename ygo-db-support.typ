#let title = "DB系列首批及续投本家字段卡数量统计"
#let author = "arshtyi"
#let date = datetime.today()
#set document(title: title, author: author, date: date)
#set page(margin: 20pt, height: auto, width: 29cm)
#set text(lang: "zh", region: "cn", font: ("Noto Serif CJK SC", "Noto Sans CJK SC"))
#align(center)[
    #text(size: 20pt, title)\
    包括卡名、效果带有字段或特定卡名的卡，不包括进入DB前的老卡，不包括之后的异画、散卡、衍生物
]
#let data = yaml("data/db.yaml")
#let data = {
    let kvs = (:)
    for (name, arr) in data {
        let image = arr.image
        let count = arr.count
        let key = str(eval(mode: "code", count))
        kvs.insert(key, kvs.at(key, default: ()) + ((name: name, image: image, count: count),))
    }
    kvs.pairs().sorted(key: pair => pair.at(0), by: (a, b) => a >= b).to-dict()
}
#let len = 0
#for (count, archs) in data { if archs.len() > len { len = archs.len() + 1 } }
#let cells = {
    let cells = ()
    for (count, archs) in data {
        let count = int(count)
        let row = ()
        let cell = if count != 0 { text(20pt)[#count 张] } else { text(20pt)[无字段] }
        row.push(cell)
        for arch in archs {
            cell = [
                #set align(center)
                #stack(
                    spacing: 4pt,
                    image("image/" + str(arch.image) + ".jpg", height: 80pt, width: 80pt),
                    text(10pt)[#arch.name #if count != 0 { arch.count }],
                )
            ]
            row.push(cell)
        }
        if row.len() < len {
            row = row + ([],) * (len - row.len())
        }
        cells += row
    }
    cells
}
#table(
    columns: (auto,) + (1fr,) * (len - 1),
    align: left + horizon,
    column-gutter: (0pt, 0pt),
    row-gutter: 0pt,
    stroke: none,
    ..cells
)
