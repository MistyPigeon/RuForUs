#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// This is a basic example. For real OneDrive API, use Microsoft Graph API (requires OAuth).
// Here we simply copy a file to the OneDrive local folder.

int main() {
    const char *onedrive_path = getenv("USERPROFILE");
    char dest[512];

    if (!onedrive_path) {
        fprintf(stderr, "USERPROFILE not found. Are you on Windows?\n");
        return 1;
    }
    // Adjust to match your OneDrive Personal path
    snprintf(dest, sizeof(dest), "%s\\OneDrive\\DatRainCacheTest.txt", onedrive_path);

    printf("Copying file to: %s\n", dest);
    // Touch a test file in OneDrive
    FILE *f = fopen(dest, "w");
    if (!f) {
        fprintf(stderr, "Failed to open file in OneDrive folder.\n");
        return 1;
    }
    fprintf(f, "This is a DatRain sync test.\n");
    fclose(f);
    printf("File created in OneDrive folder. It should sync automatically.\n");
    return 0;
}
