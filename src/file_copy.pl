use strict;
use warnings;
use File::Copy;

# Simple file copy: perl file_copy.pl source.txt destination.txt
my ($src, $dst) = @ARGV;

unless ($src && $dst) {
    print STDERR "Usage: perl file_copy.pl <source_file> <destination_file>\n";
    exit 1;
}

if (copy($src, $dst)) {
    print "Copied $src to $dst\n";
} else {
    print STDERR "Copy failed: $!\n";
    exit 1;
}
