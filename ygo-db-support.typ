#let title = "DB系列首批及续投本家字段卡数量统计"
#let author = "arshtyi"
#let date = datetime.today()
#set document(title: title, author: author, date: date)
#set text(lang: "zh", region: "cn", font: ("Noto Serif CJK SC", "Noto Sans CJK SC"))
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
#let len = 1
#for (count, archs) in data { len = calc.max(len, archs.len() + 1) }
#let image-size = 80pt
#let page-margin = 20pt
#set page(margin: page-margin, height: auto, width: len * image-size + 2 * page-margin)
#align(center)[
    #text(size: 20pt, title)\
    包括卡名、效果带有字段或特定卡名的卡，不包括进入DB前的老卡，不包括之后的异画、散卡、衍生物
]
#let cells = {
    let cells = ()
    for (count, archs) in data {
        let count = int(count)
        let row = ()
        let cell = if count != 0 { text(20pt)[#count 张] } else { text(20pt)[无字段] }
        row.push(cell)
        for arch in archs {
            cell = [#stack(
                spacing: 4pt,
                image("image/" + str(arch.image) + ".jpg", height: image-size, width: image-size),
                text(10pt)[#arch.name #if count != 0 { arch.count }],
            )]
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
    columns: (image-size,) * len,
    align: (x, y) => if x == 0 { left + horizon } else { center + top },
    inset: 0pt,
    column-gutter: 0pt,
    row-gutter: 3pt,
    stroke: none,
    ..cells
)
