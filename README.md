# Rust Qat Solver

Inspired by a [recent CodeGolf](https://codegolf.stackexchange.com/questions/259453), I discovered that there's no Qat Solver crate for Rust.
This is a WIP Rust Crate for working with [Qat](https://www.quinapalus.com/qat.html) word patterns.
The project is still in the Proof-of-Concept phase; beware of bugs, unstable APIs, and poor performance.
Contributions welcome.

## Qat

Here are the supported operations of Qat expressions.
For the proof of concept, only English will be supported.

| Task | Pattern | Results | Supported? |
|------|---------|---------|------------|
| Complete a crossword entry | `l.......v` | leitmotiv lermontov | Yes |
| Complete a crossword entry with alternatives | `..i[sz]e` | anise arise avise avize baize ... | Yes |
| Match ranges of letters | `[l-p].[m-r].[w-z]` | lammy lanky lapaz lardy larky larry lenny limax ... | No |
| Match disallowing certain letters | `....i[!stz]e` | abusive acarine acquire adenine affaire airline airmile airside ... | Yes |
| Match consonants and vowels | `#@#@#@#@#@#@#@` | pararosaniline recapitulative rehabilitative supererogative verisimilitude | No |
| Find all words beginning with `xo` | `xo*` | xosa xoana xoanon | No |
| Find all words beginning with `x` and ending in `a` | `x*a` | xema xosa xenia xhosa xoana xeroma xyloma xerasia xylopia xanthoma ... | No |
| Find all words containing the sequence `xj` | `*xj*` | vieuxjeu boxjunction boxjellyfish boxjunctions knoxjohnston | No |
| Find all words with a `j` as the second letter, followed somewhere by a `k` | `.j*k*` | sjambok djakarta sjamboks sjambokked sjambokking | No |
| Find all words which contain the vowels in order | `*a*e*i*o*u*` | caesious arsenious facetious haveitout parecious abstemious aeruginous ... | No |
| Find all words which contain five consecutive vowels | `*@@@@@*` | euouae zooeae euouaes miaoued cooeeing miaouing queueing queueings | No |
| Find all words which consist of `ace` followed by a word | `ace>` | acetic acerate acerose acetate acetone acetose acescent acetates acetones acetabula acetabular | No |
| Find all words of at most six letters beginning with `x` and ending in `a` | `-6:x*a` | xema xosa xenia xhosa xoana xeroma xyloma | No |
| Find all words of seven to nine letters beginning with `x` and ending in `a` | `7-9:x*a` | xerasia xylopia xanthoma xenophya xeromata xylomata xanthoura xenarthra xenomania xeroderma xerostoma xiphosura xylophaga xylorimba | No |
| Find all words of at least ten letters beginning with `x` and ending in `a` | `10-:x*a` | xanthomata xanthopsia xenophobia xerodermia xerostomia xenoglossia xanthochroia xanthochromia xerophthalmia | No |
| Find all words of exactly nine letters which, when the first and last letters are deleted, makes a reversed word | `9:.<.` | banisters canisters denitrate ... xenograft | No |
| Find all words one letter away from `bonge` | `` `bonge`` | binge bodge bonce bongo bongs bonne bonze bouge conge longe | No |
| Complete a crossword entry where one of the crossing letters may be wrong | ``?`str.g.ly`` | scraggly scriggly stingily stodgily straggle straggly straitly strictly strigils strongly struggle | No |
| Find all anagrams of triangle | `/triangle` | alerting altering integral relating tanglier triangle | No |
| Find all anagrams of triangle plus one extra letter | `/triangle.` | clareting earthling faltering gnarliest ... | No |
| Find words of at least seven letters made from given letters | `7-:*/rpoyesdif` | perfidy periods prosify spidery | No |
| Find all ten-letter words starting with `q` and ending in `s` which can be made from the letters of `square peg in a round hole` | `10:q*s/squarepeginaroundhole` | quadruples quandaries quarendens quarenders queenhoods quinapalus | No |
| Find all words that contain a `q`, an `x`, and a `z` | `/qxz*` | squeezebox squeezeboxes | No |
| Find all words of at most eight letters that contain all the vowels | `-8:/aeiou*` | douleia eulogia miaoued moineau sequoia aboideau aboiteau aurevoir ... | No |
| Find all words with a `j` as the penultimate or last letter | `*j. \| *j` | j jo gju haj raj taj baju benj dojo fiji fuji gajo hadj haji hajj juju mojo ... | No |
| Find all words starting with `kn` that are also words written backwards | `kn* & <` | knar knit knob knot know knub knaps knits knots knuts | No |
| Find all words of at least twelve letters not containing a reversed sub-word | `12-:! *<*` | drinkdriving flyingcircus gainstriving kicksywicksy offscourings quickthinking | No |
| Find all words starting or ending with `b`, no initial segment of which is a word | `! >*. & (b* \| *b)` | b ab ba be ... bwana bwazi climb clomb droob dweeb hbomb jacob jelab kebab kebob scrab scrub squab squib thumb vocab aplomb bhagee bhajee ... | No |

## License

This library implementing the Qat specification is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

The specification defining the Qat language belongs to Dr. Mark Owen (<https://www.quinapalus.com/>) and is licensed under the Lesser General Public License For Linguistic Resources (LGPL-LR or <http://infolingu.univ-mlv.fr/DonneesLinguistiques/Lexiques-Grammaires/lgpllr.html>).
