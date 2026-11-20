# susfs selinux_hide feature
# old susfs won't have this manual hook
# and nongki 4.14- can't use simonpunk's hook for that
# check it, when there doesn't have, fallback to auto hook
ifeq ($(shell grep -q "ksu_selinux_hide_running" $(srctree)/security/selinux/hooks.c; echo $$?),0)
$(info -- $(REPO_NAME)/susfs_feature_check: selinux_hide manual hook found)
ccflags-y += -DKSU_COMPAT_HAS_SUSFS_FEATURE_SELINUX_HIDE
endif

# susfs's dev branch currently using post_execve_hook
# but in other branch, it still direcctly call ksu_install_su_fd
# to avoid install su fd repeatedly
ifeq ($(shell grep -q "ksu_install_su_fd" $(srctree)/fs/exec.c; echo $$?),0)
$(info -- $(REPO_NAME)/compat: ksu_install_su_fd direct call found)
ccflags-y += -DKSU_COMPAT_HAS_SUSFS_INSTALL_SU_FD_DIRECT_CALL
endif

# The 2.4 API replaces the legacy try_umount/memfd commands with UID filters.
ifeq ($(shell grep -q '^\#define CMD_SUSFS_SET_SUS_PATH_UID_FILTER' $(srctree)/include/linux/susfs_def.h; echo $$?),0)
ccflags-y += -DKSU_COMPAT_HAS_SUSFS_V24
$(info -- $(REPO_NAME)/susfs: 2.4 UID-filter API detected)
endif

ifeq ($(shell grep -q '^\#define SUSFS_KAZESU_LEGACY_EXTENSIONS' $(srctree)/include/linux/susfs_def.h; echo $$?),0)
ccflags-y += -DKSU_COMPAT_HAS_SUSFS_LEGACY_EXTENSIONS
$(info -- $(REPO_NAME)/susfs: legacy try_umount and memfd extensions detected)
endif
