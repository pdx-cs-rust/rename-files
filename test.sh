#!/bin/sh
PROG=`pwd`/target/debug/rename-files
DIR=/tmp/rename-files-test.$$
cargo build &&
trap "rm -rf $DIR" 0 1 2 3 15 &&
mkdir $DIR &&
cd $DIR &&
echo 0 >file0.txt &&
echo 1 >file1.txt &&
$PROG '[0-9]+' '0$0' *.txt

if [ $? -ne 0 ] || [ `ls | wc -l` -ne 2 ] || [ ! -f file00.txt ] || [ ! -f file01.txt ]
then
    echo "failed ($DIR)"
    trap "" 0
    exit 1
else
    echo "succeeded"
fi

