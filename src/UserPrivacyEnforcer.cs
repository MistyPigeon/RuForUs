using System;
using System.IO;
using System.Security.AccessControl;
using System.Security.Principal;

namespace DatRain
{
    public class UserPrivacyEnforcer
    {
        /// <summary>
        /// Sets file or directory permissions so that only the current user can access the specified path.
        /// Works on Windows using NTFS permissions.
        /// </summary>
        /// <param name="path">File or directory to protect</param>
        public static void ProtectPath(string path)
        {
            if (!File.Exists(path) && !Directory.Exists(path))
            {
                Console.WriteLine($"Path does not exist: {path}");
                return;
            }

            var user = WindowsIdentity.GetCurrent().User;
            if (user == null)
            {
                Console.WriteLine("Could not get current user SID.");
                return;
            }

            if (File.Exists(path))
            {
                var fileInfo = new FileInfo(path);
                var security = fileInfo.GetAccessControl();
                security.SetAccessRuleProtection(true, false); // Disable inheritance
                security.SetOwner(user);

                // Remove all rules
                foreach (FileSystemAccessRule rule in security.GetAccessRules(true, true, typeof(SecurityIdentifier)))
                {
                    security.RemoveAccessRule(rule);
                }

                // Grant full control to current user only
                var accessRule = new FileSystemAccessRule(
                    user,
                    FileSystemRights.FullControl,
                    AccessControlType.Allow
                );
                security.AddAccessRule(accessRule);

                fileInfo.SetAccessControl(security);
                Console.WriteLine($"File protected: {path}");
            }
            else if (Directory.Exists(path))
            {
                var dirInfo = new DirectoryInfo(path);
                var security = dirInfo.GetAccessControl();
                security.SetAccessRuleProtection(true, false); // Disable inheritance
                security.SetOwner(user);

                // Remove all rules
                foreach (FileSystemAccessRule rule in security.GetAccessRules(true, true, typeof(SecurityIdentifier)))
                {
                    security.RemoveAccessRule(rule);
                }

                // Grant full control to current user only
                var accessRule = new FileSystemAccessRule(
                    user,
                    FileSystemRights.FullControl,
                    InheritanceFlags.ContainerInherit | InheritanceFlags.ObjectInherit,
                    PropagationFlags.None,
                    AccessControlType.Allow
                );
                security.AddAccessRule(accessRule);

                dirInfo.SetAccessControl(security);
                Console.WriteLine($"Directory protected: {path}");
            }
        }

        // Example usage
        public static void Main(string[] args)
        {
            if (args.Length != 1)
            {
                Console.WriteLine("Usage: UserPrivacyEnforcer <file-or-directory-path>");
                return;
            }
            ProtectPath(args[0]);
        }
    }
}
