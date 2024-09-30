# Yet Another MKdir/Touch

## ALL CREDITS DUE TO [THE ORIGINAL PROJECT](https://github.com/khalil-benkirat/yamkt)

---

 ### YAMKT is a mkdir/touch replacement, it basically is a `mkdir -p` but more sophisticated

---

## Examples:

Here are some examples and the folder architecture after executing the commands
(all the base folders are considered empty before executing the command):

```sh
$ yamkt "dir1/dir2/file1.txt"
$ tree
dir1/
  dir2/
    file1.txt
```

```sh
$ yamkt "(dir1.1,dir1.2)/file.txt"
$ tree
dir1.1/
  file.txt
dir1.2/
  file.txt
```

```sh
$ yamkt "dir1/(file1.txt,file2.txt)"
$ tree
dir1/
  file1.txt
  file2.txt
```

```sh
$ yamkt "dir1/dir2/(dir3.1,dir3.2)/"
$ tree
dir1/
  dir2/
    dir3.1/
    dir3.2/
```

```sh
$ yamkt "(dir1.1,dir1.2)/(file1.txt,file2.txt)"
$ tree
dir1.1/
  file1.txt
  file2.txt
dir1.2/
  file1.txt
  file2.txt
```

```sh
$ yamkt "dir1/(dir2,dir3/file1.txt)/dir4/file2.txt"
$ tree
dir1/
  dir2/
    dir4/
      file2.txt
  dir3/
    file1.txt
```

---

## Important:
Expressions cannot start with a `/`</br>
Expressions should not contains two consecutive `/`</br>

The outer separator is `/`</br>
The inner separator (to separate files or directories in parenthesis) is `,`</br>

Therefore, you can't create files or folders with a `/` nor with a `,` in their name</br>


---

## WARNING:
Watch out what you run, this can easily take up a lot of space quickly.</br>
For example:
```sh
$ yamkt "(dir1,dir2)/(dir3,dir4)/(dir5,dir6)/(dir7,dir8)/(dir9,dir10)/(dir11,dir12)/(dir13,dir14)/(dir15,dir16)/(dir17,dir18)/(dir19,dir20)/(dir21,dir22)/(dir23,dir24)/(dir25,dir26)/(dir27,dir28)/(dir29,dir30)/(dir31,dir32)/(dir33,dir34)/(dir35,dir36)/(dir37,dir38)/(dir39,dir40)/"
```
</br>Creates about 20 millions directories, or 80GB of space taken since a Linux empty directory is 4Kb
