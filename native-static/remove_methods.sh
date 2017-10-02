#!/bin/bash

c=$PWD/target/thumbv7m-linux-eabi/$1/libIOTA.a
cb=`ar t $c | grep compiler`

pushd  .
tmpdir=`mktemp -d`

cd $tmpdir
ar x $c $cb

arm-none-eabi-objcopy -L __aeabi_dadd $cb
arm-none-eabi-objcopy -N __aeabi_dsub $cb
arm-none-eabi-objcopy -N __aeabi_fsub $cb
arm-none-eabi-objcopy -L __aeabi_fadd $cb
arm-none-eabi-objcopy -N __aeabi_ui2f $cb
arm-none-eabi-objcopy -N __aeabi_i2f $cb
arm-none-eabi-objcopy -N __aeabi_ui2d $cb
arm-none-eabi-objcopy -N __aeabi_i2d $cb
arm-none-eabi-objcopy -N __aeabi_ul2d $cb
arm-none-eabi-objcopy -N __aeabi_l2d $cb
arm-none-eabi-objcopy -N __aeabi_d2iz $cb
arm-none-eabi-objcopy -N __aeabi_f2iz $cb
arm-none-eabi-objcopy -N __aeabi_f2uiz $cb
arm-none-eabi-objcopy -L __aeabi_uldivmod $cb
arm-none-eabi-objcopy -L __aeabi_ldivmod $cb
arm-none-eabi-objcopy -N __divdi3 $cb
arm-none-eabi-objcopy -N __udivdi3 $cb
arm-none-eabi-objcopy -N memcmp $cb
arm-none-eabi-objcopy -N memcpy $cb
arm-none-eabi-objcopy -N memmove $cb
arm-none-eabi-objcopy -N memset $cb

ar r $c $cb
rm $cb
popd

rmdir $tmpdir
