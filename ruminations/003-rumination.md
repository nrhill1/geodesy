# Ruminations on Rust Geodesy

## Rumination 003: `kp` - the RG Coordinate Processing program

Thomas Knudsen <knudsen.thomas@gmail.com>

2021-08-26. Last [revision](#document-history) 2021-08-26

---

### Prologue

`kp` is the Rust Geodesy **coordinate processing** program. The obvious abbreviation of coordinate processing is `cp`, but since `cp` is the Unix file copying program we substitute k for c - hence `kp`, which may be taken as a reference to the Danish word for coordinate processing **koordinatprocessering**.

Incidentally, `kp` was also the user-id and email address of the late **Knud Poder** (1925-2019), during his work years at the Danish geodetic institute, GI (and its successor, KMS), from the 1950s until his retirement in 1995.

For many years, Poder was in charge of the GI department for computational geodesy where, for some years around 1980, his deputy was Carl Christian Tscherning (1942-2014), for whom the [PROJ](https::/proj.org) transformation program [cct](https://proj.org/apps/cct.html) was named. Among friends, colleagues and collaborators worldwide, Knud Poder was regarded a "Nestor of computational geodesy.

### Usage

`kp` is still under heavy development, but so far the command line syntax is:

```sh
kp "operation" file1 file2 ...
```

or, with input from `stdin`:

```sh
echo coordinate |  kp "operation"
```

**Example:**
Convert the geographical coordinate tuple (55 N, 12 E) to utm, zone 32 coordinates:

```sh
echo 55 12 0 0 | kp "geo | utm zone:32"
> 691875.6321 6098907.8250 0.0000 0.0000
```

While RG coordinates are always 4D, `kp` will provide zero-values for any left-out postfix dimensions:

```sh
echo 55 12 | kp "geo | utm zone:32"
> 691875.6321 6098907.8250 0.0000 0.0000
```

### Operators

The current crop of RG operators are described in the [missing manual](/ruminations/002-rumination.md)

### A few more words about Knud Poder

On the occasion of Knud Poder's 90th birthday in 2015, I wrote a few words about one of his accomplishments on the [PROJ mailing list](https://lists.osgeo.org/pipermail/proj/2015-October/006884.html):

```txt
As described in a recent thread, for the next release, proj.4 will switch
the default transverse mercator implementation from tmerc to etmerc.

This is probably a good occasion to reiterate the history of the code for
the etmerc implementation - especially since the original author, Knud
Poder, turned 90 on October 19th. Having his transverse mercator
implementation becoming the proj.4 default is a strikingly proper way of
celebrating Poder, among colleagues and collaborators rightfully considered
“the Nestor of computational geodesy”.

Poder wrote the first version of what is now known as etmerc, around 1961.
It was written in Algol-60 and ran on the GIER computer, built for the
Danish Geodetic Institute (see [1] for details).

The code was based on theoretical foundations published a decade earlier,
by König & Weise ([2], building on prior work by Krüger, 1912 [3]).

Poder’s work was characterized by great care with respect to numerical
precision and accuracy (e.g. by using Clenshaw summation for recurrence
series, and Horner’s scheme for polynomial evaluation).

Also, Poder was noted for his ingeniously implemented “dual autochecking
method” (not used in the proj.4 version), where the same code was used for
forward and inverse projections and was run both ways and compared, to
protect against both coding- and hardware errors. The latter was very
important at a time where the mean time between failure for computer
systems was much shorter than today.

During the 1970s Poder’s student, Karsten Engsager (the “E” in etmerc,
“Engsager Extended Transverse Mercator”) took over maintenance and
eventually extended König and Weise’s numerical series by another term,
bringing the accuracy up to today’s standard.

In 2008, through the efforts of a.o. Gerald Evenden, Frank Warmerdam and
Karsten Engsager, etmerc was introduced in proj.4, while in 2013 Charles
Karney provided 3 corrections - stressing the value and importance of open
source code sharing.

Poder retired 20 years ago, but has been taking active interest in the
maintenance and development of his code ever since. Switching proj.4 to use
a transverse mercator implementation based on his work is probably the best
conceivable way of celebrating the 90th birthday of a great Nestor of
computational geodesy.

In celebration of Knud Poder!

/Thomas Knudsen, Danish Geodata Agency


[1] Thomas Knudsen, Simon L. Kokkendorff, Karsten E. Engsager (2012): A
Vivid Relic Under Rapid Transformation, OSGeo Journal vol. 10, pp. 55-57,
URL https://journal.osgeo.org/index.php/journal/article/download/200/167

[2] R. König and K. H. Weise (1951): Mathematische Grundlagen der Höheren
Geodäsie und Kartographie, Erster Band. Springer,
Berlin/Göttingen/Heidelberg, 1951. K

[3] L. Krüger (1912): Konforme Abbildung des Erdellipsoids in der Ebene.
Neue Folge 52. Royal Prussian Geodetic Institute, Potsdam. URL
http://bib.gfz-potsdam.de/pub/digi/krueger2.pdf
```

### Document History

Major revisions and additions:

- 2021-08-28: Initial version