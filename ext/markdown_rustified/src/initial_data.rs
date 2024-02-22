pub(super) fn load() -> String {
    String::from(
        r#"
## タイトル１
++this is something important++
hello world

## タイトル２
ぶらぶらぶら

### タイトル２-１
hogehogeほげほげ
hogehogeほげほげ
hogehogeほげほげ
hogehogeほげほげ
[リンク1](https://www.city.fuji.shizuoka.jp)
[リンク2](htts:/www.city.fuji.shizuoka.jp)
[リンク3](https://www.google.com)
[リンク4](https://google.com)
[リンク5](/foo/bar/hoge.pdf)
[リンク6](google.com/foo/bar)
ほげ ![画像タイトル](https://www.city.fuji.shizuoka.jp/page/gazou/fmervo000001dsro-att/1226-0009big.jpg) ふが
<div> HTMLタグを直接入れる </div>

[[foo bar]]

> 吾輩は猫である。名前はまだ無い。
> どこで生れたかとんと見当がつかぬ。
> [hoge](https://www.city.fuji.shizuoka.jp)
> -- <cite>夏目漱石</cite>

### タイトル２-２
- hogehogeほげほげ1
  - hogehogeほげほげ0-1
  - hogehogeほげほげ1-2
- hogehogeほげほげ2
- hogehogeほげほげ3


1. hogehogeほげほげ1
1. hogehogeほげほげ2
1. hogehogeほげほげ3

#### タイトル２-２-1
小タイトルに続く本文
hogehogeほげほげ
hogehogeほげほげ

## タイトル３
こんにちは
~~これは間違いです~~（未実装）

| title       | title        | title         |
|:-----------|------------:|:------------:|
| This       | This        | This         |
| column     | column      | column       |
| will       | will        | will         |
| be         | be          | be           |
| left       | right       | center       |
| ali<br/>gned    | aligned     | aligned      |
| will            | will                | **太字だよ** [hoge](google.com)          |

## タイトル４
タイトルに続く本文

#### 4-1-1
小タイトルに続く本文

## タイトル５
++赤字で重要なことを強調++
**太字で重要なことを強調**

==ハイライトしたい部分== をこのようにするよ

### タイトル５-１
__アンダーラインをどうするか__

#### タイトル５-１-１
じゅげむじゅげむ五劫の擦り切れ

# タイトル６-h1
## タイトル６-h2
### タイトル６-h3
#### タイトル６-h4
##### タイトル６-h5
###### タイトル６-h6
"#,
    )
}
