Check your AD Passwords against the Have I Been Pwned hashes.

1. Download the hibp ntlm hashes, ordered by count from https://haveibeenpwned.com/Passwords
2. Get your NTLM hashes from AD:
    1. On a domain controller run:
      ```
      >ntdsutil
      ntdsutil: activate instance ntds
      ntdsutil: ifm
      ifm: create full c:\audit
      ifm: quit
      ntdsutil: quit
      ```
    2. Copy the files from c:\audit on the DC to your workstation.
    3. Download ntdsaudit from https://github.com/Dionach/NtdsAudit/releases
    4. Create a CSV of users and hashes:
    ```
    NtdsAudit.exe "audit\Active Directory\ntds.dit" -s "audit\registry\SYSTEM" -p "dump.txt"
    ```
3. Run ad-audit
```
./audit-ad -d dump.txt -h pwned-passwords-ntlm-ordered-by-count-v5.txt
```
