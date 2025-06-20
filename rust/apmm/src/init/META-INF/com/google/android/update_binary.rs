/// 生成 META-INF/com/google/android/update-binary 脚本
pub fn generate_update_binary() -> String {
    r#"#!/sbin/sh
##########################################################################################
#
# Magisk Module Template Install Script
# by topjohnwu
#
##########################################################################################

TMPDIR=/dev/tmp
INSTALLER=$TMPDIR/install
# Always mount under tmp
MOUNTPATH=$TMPDIR/magisk_img

# Default permissions
umask 022

# Initial cleanup
rm -rf $TMPDIR 2>/dev/null
mkdir -p $INSTALLER

# echo before loading util_functions
ui_print() { echo "$1"; }

require_new_magisk() {
  ui_print "*******************************"
  ui_print " Please install Magisk v17.0+! "
  ui_print "*******************************"
  exit 1
}

##########################################################################################
# Environment

abort() { ui_print " "; ui_print "$*"; exit 1; }

baksmali() {
  ANDROID_DATA=$ap ANDROID_ROOT=/system LD_LIBRARY_PATH=/system/lib dalvikvm -Xbootclasspath:/system/framework/core.jar:/system/framework/conscrypt.jar:/system/framework/apache-xml.jar -classpath $baksmali org.jf.baksmali.main -o classout $1;
  test $? != 0 && abort "Decompiling APK classes failed. Aborting...";
}
smali() {
  ANDROID_DATA=$ap ANDROID_ROOT=/system LD_LIBRARY_PATH=/system/lib dalvikvm -Xbootclasspath:/system/framework/core.jar:/system/framework/conscrypt.jar:/system/framework/apache-xml.jar -classpath $smali org.jf.smali.main -o classes.dex classout;
  test $? != 0 && abort "Rebuilding APK classes failed. Aborting...";
}
apktool_d() {
  ANDROID_DATA=$ap ANDROID_ROOT=/system LD_LIBRARY_PATH=/system/lib dalvikvm -Xbootclasspath:/system/framework/core.jar:/system/framework/conscrypt.jar:/system/framework/apache-xml.jar -classpath $apktool brut.apktool.Main d --frame-path $ap/framework --no-src -o resout $1;
  test $? != 0 && abort "Decoding APK resources failed. Aborting...";
}
apktool_b() {
  ANDROID_DATA=$ap ANDROID_ROOT=/system LD_LIBRARY_PATH=/system/lib dalvikvm -Xbootclasspath:/system/framework/core.jar:/system/framework/conscrypt.jar:/system/framework/apache-xml.jar -classpath $apktool brut.apktool.Main b --frame-path $ap/framework --aapt $bin/aapt --copy-original -o $1 resout;
  test $? != 0 && abort "Rebuilding APK resources failed. Aborting...";
}

##########################################################################################

OUTFD=$2
ZIP=$3

mount /data 2>/dev/null

# Load utility functions
if [ -f /data/adb/magisk/util_functions.sh ]; then
  . /data/adb/magisk/util_functions.sh
elif [ -f /data/magisk/util_functions.sh ]; then
  NVBASE=/data
  . /data/magisk/util_functions.sh
else
  require_new_magisk
fi

# Use alternative image if in BOOTMODE
$BOOTMODE && IMG=$NVBASE/magisk_merge.img

# Preperation for flashable zips
setup_flashable

# Mount partitions
mount_partitions

# Detect version and architecture
api_level_arch_detect

# You can get the Android API version from $API, the CPU architecture from $ARCH
# Useful if you are creating Android version / platform dependent mods

# Setup busybox and binaries
$BOOTMODE && boot_actions || recovery_actions

##########################################################################################
# Preparation
##########################################################################################

# Extract common files
unzip -o "$ZIP" module.prop config.sh 'common/*' -d $INSTALLER >&2

[ ! -f $INSTALLER/config.sh ] && abort "! Unable to extract zip file!"
# Load configurations
. $INSTALLER/config.sh

# Check the installed magisk version
MIN_VER=`grep_prop minMagisk $INSTALLER/module.prop`
[ ! -z $MAGISK_VER_CODE -a $MAGISK_VER_CODE -ge $MIN_VER ] || require_new_magisk
MODID=`grep_prop id $INSTALLER/module.prop`
MODPATH=$MOUNTPATH/$MODID

# Print mod name
print_modname

# Please leave this message in your flashable zip for credits :)
ui_print "******************************"
ui_print "Powered by Magisk (@topjohnwu)"
ui_print "******************************"

##########################################################################################
# Install
##########################################################################################

# Get the variable reqSizeM. Use your own method to determine reqSizeM if needed
request_zip_size_check "$ZIP"

# This function will mount $IMG to $MOUNTPATH, and resize the image based on $reqSizeM
mount_magisk_img

# Create mod paths
rm -rf $MODPATH 2>/dev/null
mkdir -p $MODPATH

# Extract files to system. Use your own method if needed
ui_print "- Extracting module files"
unzip -o "$ZIP" 'system/*' -d $MODPATH >&2

# Remove placeholder
rm -f $MODPATH/system/placeholder 2>/dev/null

##########################################################################################
# APK-Patcher Main
##########################################################################################

# working directory variables
ap=/tmp/apkpatcher;
bin=$ap/tools;
patch=$ap/patch;
script=$ap/script;

mkdir -p $ap;
unzip -o "$ZIP" -d $ap;
if [ $? != 0 -o -z "$(ls $ap)" ]; then
  abort "Unzip failed. Aborting...";
fi;

# set up extracted files and directories
chmod -R 755 $bin $script $ap/*.sh;

# dexed bak/smali and apktool jars (via: dx --dex  --min-sdk-version=26 --output=classes.dex <file>.jar)
baksmali=$bin/baksmali-*-dexed.jar;
smali=$bin/smali-*-dexed.jar;
apktool=$bin/apktool_*-dexed.jar;

# import variables
. $ap/envvar.sh;

ui_print "Patching...";
cd $ap;
amount=$((100 / `echo $apklist | wc -w`));
subamt=$(awk -v num=$amount 'BEGIN { print num / 10}');
for apkp_target in $apklist; do
  apkname=$(basename $apkp_target .apk);
  ui_print "- Patching $apkname";

  # copy in apkp_target system file to patch
  sysfile=`find /system -mindepth 2 -name $apkp_target`;
  cp -fp $sysfile $ap;

  # make a backup if set
  if [ "$backup" == 1 ]; then
    mkdir -p $apkbak;
    cp -fp $sysfile $apkbak;
  fi;

  # smali file patches
  if [ -f $script/$apkname-smali.sh -o -d $patch/$apkname-smali ]; then
    baksmali $apkp_target;
    if [ -f $script/$apkname-smali.sh ]; then
      . $script/$apkname-smali.sh;
    fi;
    if [ -d $patch/$apkname-smali ]; then
      cp -rf $patch/$apkname-smali/* classout/;
    fi;
    smali;
  fi;
  # don't directly add to zip if there are apktool resource patches to perform
  if [ ! -f $script/$apkname-res.sh -o ! -d $patch/$apkname-res ]; then
    $bin/zip -v $apkp_target classes.dex;
    test $? != 0 && abort "Updating APK classes failed. Aborting...";
  fi;

  # resource file patches
  if [ -f $script/$apkname-res.sh -o -d $patch/$apkname-res ]; then
    apktool_d $apkp_target;
    if [ -f $script/$apkname-res.sh ]; then
      . $script/$apkname-res.sh;
    fi;
    if [ -d $patch/$apkname-res ]; then
      cp -rf $patch/$apkname-res/* resout/;
    fi;
    # add the new classes.dex from smali if it exists
    if [ -f $ap/classes.dex ]; then
      cp -f classes.dex resout/classes.dex;
    fi;
    apktool_b $apkp_target;
  fi;

  # zipalign updated file
  cp -f $apkp_target $apkname-preopt.apk;
  $bin/zipalign -p 4 $apkname-preopt.apk $apkp_target;

  # copy patched file back to system
  cp -fp $ap/$apkp_target $MODPATH$sysfile;

  # remove temp files if cleanup is set
  if [ "$cleanup" == 1 ]; then
    rm -rf classout classes.dex resout $apkp_target $apkname-preopt.apk;
  fi;
done;

# extra required non-patch changes
. $ap/extracmd.sh;

# cleanup as necessary
if [ "$cleanup" == 1 ]; then
  cd /tmp;
  rm -rf $ap;
fi;

##########################################################################################
# Back to Magisk
##########################################################################################

# Handle replace folders
for TARGET in $REPLACE; do
  mktouch $MODPATH$TARGET/.replace
done

# Auto Mount
$AUTOMOUNT && touch $MODPATH/auto_mount

# prop files
$PROPFILE && cp -af $INSTALLER/common/system.prop $MODPATH/system.prop

# Module info
cp -af $INSTALLER/module.prop $MODPATH/module.prop
if $BOOTMODE; then
  # Update info for Magisk Manager
  mktouch /sbin/.core/img/$MODID/update
  cp -af $INSTALLER/module.prop /sbin/.core/img/$MODID/module.prop
fi

# post-fs-data mode scripts
$POSTFSDATA && cp -af $INSTALLER/common/post-fs-data.sh $MODPATH/post-fs-data.sh

# service mode scripts
$LATESTARTSERVICE && cp -af $INSTALLER/common/service.sh $MODPATH/service.sh

ui_print "- Setting permissions"
set_permissions

##########################################################################################
# Finalizing
##########################################################################################

# Unmount magisk image and shrink if possible
unmount_magisk_img

$BOOTMODE || recovery_cleanup
rm -rf $TMPDIR

ui_print "- Done"
exit 0
"#.to_string()
}
