# Yet Another MKdir/Touch

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
Expressions cannot start with a `/`
Expressions should not contains two consecutive `/`

The outer separator is `/`
The inner separator (to separate files or directories in parenthesis) is `,`

Therefore, you can't create files or folders with a `/` nor with a `,` in their name
