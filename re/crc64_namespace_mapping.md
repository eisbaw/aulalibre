# CRC64 Hash to .NET Namespace Mapping

Mapping of Xamarin Android Callable Wrapper (ACW) CRC64 hashed Java package names
back to their original .NET namespace/class names.

## Methodology

Each ACW smali class contains a static constructor that calls
`mono.android.Runtime.register("Full.Dotnet.TypeName, AssemblyName", ...)`.
We extract these registrations to build a definitive mapping from CRC64 hashes
to .NET namespaces.

## Statistics

- **Total CRC64 hashes found in smali**: 179
- **Total .NET type registrations**: 677
- **CRC64 hashes in AndroidManifest.xml**: 72
- **Hashes matched (in both)**: 72
- **Hashes in manifest only**: 0
- **Hashes in smali only**: 107

## Complete Mapping Table

| CRC64 Hash | .NET Namespace | Assembly | Classes |
|------------|----------------|----------|----------|
| `crc640056e568e07a5371` | `AulaNative.Droid.CustomViews.AulaSwitch` | AulaNative.Droid | AulaSwitch |
| `crc6401be2761ec990c52` | `AulaNative.Droid.Activities.Document.CloudIntegration.DocumentFragment` | AulaNative.Droid | CloudStorageFragment |
| `crc64056db8367489a9a5` | `AulaNative.Droid.Activities.Onboarding.Policy` | AulaNative.Droid | AulaLinkClickMovementMethod, PolicyFragment |
| `crc6407ec88ddc45e0223` | `AulaNative.Droid.Activities.Onboarding` | AulaNative.Droid | OnboardingActivity, OnboardingFragmentPagerAdapter |
| `crc6409d51c9fa7ea23bb` | `AulaNative.Droid.CustomViews.AulaTextView` | AulaNative.Droid | AulaButton, AulaTextView, AulaTextViewWithArrowLinearLayout |
| `crc640a1f4d108c17e3f1` | `Microsoft.Maui.ApplicationModel.DataTransfer` | Microsoft.Maui.Essentials | ClipboardChangeListener |
| `crc640a8d9a12ddbf2cf2` | `Microsoft.Maui.Devices` | Microsoft.Maui.Essentials | BatteryBroadcastReceiver, DeviceDisplayImplementation+Listener, EnergySaverBroadcastReceiver |
| `crc640c05e18e2e9d438c` | `AulaNative.Droid.Views.AulaStatusImage` | AulaNative.Droid | AulaStatusImage |
| `crc640c25afc07e567a01` | `AulaNative.Droid.CustomViews.ListModalBottomSheet` | AulaNative.Droid | ListModalBottomSheet, ListModalBottomSheetAdapter, ListModalBottomSheetTwoLinesViewHolder, ListModalBottomSheetViewHolder |
| `crc640c5494012a06d265` | `AulaNative.Droid.Activities.Calendar.Event.Overlapping` | AulaNative.Droid | EventViewHolder, OverLappingAdapter, OverlappingActivity, OverlappingHeaderViewHolder |
| `crc640d97cec38b77325f` | `AulaNative.Droid.Activities.Album` | AulaNative.Droid | AlbumEditFormActivity, AlbumEditFormFragment, AlbumTagFragment, FilePickerActivity, GalleryPickerActivity, ImportMediaContainerActivity, MediaSelectionFragment, MediaTagFragment |
| `crc640e895d78a388fb08` | `AulaNative.Droid.Views.MainPage` | AulaNative.Droid | AulaBottomBarEditShortcutsItemView, AulaBottomNavigationItemView, AulaMainPageBottomNavigationView |
| `crc640f83e066fd36f625` | `AulaNative.Droid.Views.DataPolicyWebView` | AulaNative.Droid | DataPolicyWebView, DataPolicyWebView+DataPolicyWebViewClient |
| `crc6410c8e6ba9ee100a4` | `AulaNative.Droid.Views.MultiAulaImageAbbreviation` | AulaNative.Droid | MultiAulaImageAbbreviation |
| `crc64117d19181bd40e67` | `AulaNative.Droid.Activities.Messages.MessageThread` | AulaNative.Droid | MessageThreadActivity, MessageThreadAdapter, MessageThreadDataAdapterObserver, MessageThreadFragment, MessageThreadFragment+ActivityResultCallback, MessageThreadFragment+ChildAttachStateChangeListener |
| `crc6412763d6b05397a23` | `AulaNative.Droid.Views` | AulaNative.Droid | AulaSearchBar |
| `crc64153da6e621868364` | `AulaNative.Droid.CustomViews.AulaFilterAndSortView` | AulaNative.Droid | AulaFilterAndSortDropdown, FilterAndSortCellHolder, FilterAndSortDropdownAdapter |
| `crc641b0203710e4b7ee0` | `AulaNative.Droid.ViewModels.Common` | AulaNative.Droid | DetailsTabBarItem`1, TabBarItem |
| `crc641e4a654bb7dceae5` | `AulaNative.Droid.Activities.ComeGoStaff.WeekOverview.PresenceChildren` | AulaNative.Droid | PresenceChildrenDistributionActivity, PresenceChildrenDistributionActivityAdapter, PresenceChildrenDistributionActivityAdapter+ContentViewHolder, PresenceChildrenDistributionActivityAdapter+HeaderViewHolder |
| `crc641fcc2ac2cfa79c06` | `AulaNative.Droid.Activities.Onboarding.MasterData` | AulaNative.Droid | MasterDataFragment, MasterDataInstitutionProfileHolder |
| `crc6421c919819d958882` | `AulaNative.Droid.CustomViews.AutoCompleteControl.SelectionPage` | AulaNative.Droid | AutoCompleteControlSelectionPageActivity, AutoCompleteControlSelectionPageActivity+SelectionPageScrollListener, AutoCompleteControlSelectionPageAdapter, AutocompleteControlSelectionPageCellViewHolder, AutocompleteControlSelectionPageCellViewHolder`1, AutocompleteControlSelectionPageSectionViewHolder |
| `crc6422aa70c17a2f84a5` | `AulaNative.Droid.Utils.RecyclerSwipeLayout.Util` | AulaNative.Droid | Attributes |
| `crc6422b55094979017ec` | `AulaNative.Droid.CustomViews.FullscreenMediaPlayer` | AulaNative.Droid | FullscreenMediaPlayerActivity, FullscreenMediaPlayerActivity+OnInfoListener, FullscreenMediaPlayerActivity+OnPreparedListener |
| `crc6426d4d8d262dda33c` | `AulaNative.Droid.Activities.Groups` | AulaNative.Droid | GroupDashboardActivity, GroupDashboardActivity+GroupDashboardPageChangeListener, GroupDashboardActivity+GroupDashboardPagerAdapter, GroupDashboardOverviewFragment, GroupMembershipActivity, GroupMembershipsListFragment, GroupsListAdapter, GroupsMenuFragment |
| `crc64276fbe2dc15076e4` | `AulaNative.Droid.CustomViews.AutoCompleteControl` | AulaNative.Droid | AutoCompleteControlEditor |
| `crc642856e7fed4fd87c9` | `AulaNative.Droid.Activities.Album.ViewHolders` | AulaNative.Droid | BaseViewHolder, MediaSelectionOtherItemViewHolder |
| `crc6428737b50d58965aa` | `AulaNative.Droid.Activities.ComeGoStaff.WeekOverview` | AulaNative.Droid | NoDataWeekOverviewItemViewHolder, PhoneWeekOverviewItemViewHolder, TabletWeekOverviewHeaderItemViewHolder, TabletWeekOverviewItemViewHolder, WeekOverviewFragment, WeekOverviewPhoneListAdapter, WeekOverviewTabletListAdapter |
| `crc642a61f038ea047fcc` | `AulaNative.Droid.Utils.Decorators` | AulaNative.Droid | DividerWithoutFirstAndLastLineItemDecorator |
| `crc642ab953f6613e37b6` | `AulaNative.Droid.Activities.Messages` | AulaNative.Droid | AdvancedSearchActivity, AlertDialogShowListener, DialogAddFolder, MessageThreadDataAdapterObserver, MessageThreadViewHolder, MessageThreadViewHolderFoldOutCell, MessageThreadViewHolderFoldOutHeaderCell, MessageViewHolder, MessagesOverviewFragment, MessagesOverviewListAdapter |
| `crc642b5ea7562757e68e` | `AulaNative.Droid.Activities.ComeGoStaff.Overview.Filter.DepartmentAndGroupsFiltering` | AulaNative.Droid | ActivityListDepartmentAndGroupsFilteringActivity, ActivityListDepartmentAndGroupsFilteringActivityAdapter, ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringContentViewHolder, ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringDepartmentViewHolder, ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringGroupViewHolder, ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringHeaderViewHolder |
| `crc642b71a4216b6636ac` | `AulaNative.Droid.Views.CalendarView` | AulaNative.Droid | EventTextView |
| `crc642db8434b2db369d4` | `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.ProfileSectionWrapper` | AulaNative.Droid | MultipleProfilesSectionWrapper+ComeGoMultipleProfileChildViewHolder, MultipleProfilesSectionWrapper+MultipleChildrenAdapter |
| `crc6433c817910c30379b` | `AulaNative.Droid.Activities.Calendar.ImportantDate` | AulaNative.Droid | ImportantDateAdapter, ImportantDateFragment, ImportantDateViewHolder, SpaceItemDecoration |
| `crc6434af9c19aa01b597` | `Android.Gms.Common.Apis` | Xamarin.GooglePlayServices.Base | GoogleApiClientConnectionCallbacksImpl, GoogleApiClientOnConnectionFailedListenerImpl |
| `crc6436b99b97cc67a1cc` | `AulaNative.Droid.Utils` | AulaNative.Droid | AccessibilityDelegate`1, AccessibilityUtils+AccessibilityValueDelegate, ActivityResultCallback, BasicHeaderViewHolder, DrawableExtension+ScaledDrawableWrapper, LinkClickableSpan, RecyclerEventViewOnScrollListener, RecyclerViewOnScrollListener, ViewTreeObserverUtils+TemporaryGlobalLayoutListener |
| `crc64379f1df1f1ef6185` | `AulaNative.Droid.Views.SelectionListView` | AulaNative.Droid | AulaSelectionHeaderViewHolder, AulaSelectionViewHolder, SelectionListAdapter, SelectionListView |
| `crc64396a3fe5f8138e3f` | `AndroidX.Browser.CustomTabs` | Xamarin.AndroidX.Browser | CustomTabsServiceConnectionImpl, KeepAliveService |
| `crc6439eb6491da0455b9` | `AulaNative.Droid.Activities.PersonalReferenceData.ViewOtherMasterDataFilteringEntry` | AulaNative.Droid | ViewOtherMasterDataFilteringEntryFragment |
| `crc643b2c14d5ceefefc8` | `AulaNative.Droid.CustomViews.AutoCompleteControl.SearchBookables` | AulaNative.Droid | BookableSelectionCellViewHolder |
| `crc643ccd8ae99eb00e6c` | `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm` | AulaNative.Droid | ComeGoActivityEditFormActivity, ComeGoGenericSelectionAdapter`1, ComeGoGenericSelectionAdapter`1+GenericSelectionViewHolder |
| `crc643f2b18b2570eaa5a` | `Microsoft.Maui.Graphics.Platform` | Microsoft.Maui.Graphics | PlatformGraphicsView |
| `crc64427d32b40b56664a` | `AulaNative.Droid.CustomViews.RoundedCornerButton` | AulaNative.Droid | RoundedCornerButton |
| `crc64439a32af6af2bcba` | `AulaNative.Droid.Activities.ComeGoStaff.VacationList` | AulaNative.Droid | VacationListOverviewFragment |
| `crc644533801b3f66659a` | `AulaNative.Droid.Activities.ActivityList.VacationList` | AulaNative.Droid | VacationItemViewHolder, VacationListAdapter |
| `crc64460bdd3cfa678830` | `AulaNative.Droid.CustomViews.TooltipButton` | AulaNative.Droid | TooltipButton |
| `crc64464655eca3cd0aac` | `AulaNative.Droid.Activities.Calendar` | AulaNative.Droid | CalendarFragment, CalendarOverviewFragment, CalendarViewPagerAdapter |
| `crc64465cc550138d14d3` | `AulaNative.Droid.Views.LoadingOverlay` | AulaNative.Droid | LoadingOverlay |
| `crc6449307630a8b601a5` | `AulaNative.Droid.Activities.Document.SecureDocuments.Revision` | AulaNative.Droid | SecureDocumentViewRevisionActivity, SecureDocumentViewRevisionAdapter, SecureDocumentViewRevisionViewHolder |
| `crc644b585a7d6893d48b` | `AulaNative.Droid.Activities.Messages.MessageFolder` | AulaNative.Droid | FolderItemViewHolder, FolderSectionItemViewHolder, MessageCreateNewFolderFragment, MessageFolderActivity, MessageFolderAdapter, MessageFolderFragment, SubFolderItemViewHolder |
| `crc644caf61526f1f2e48` | `AulaNative.Droid.Activities.ComeGo.AbsenceTab` | AulaNative.Droid | AbsenceVacationActivity, AbsenceVacationRecyclerViewAdapter, ChildSwitchCellRecyclerViewHolder, ContentCellRecyclerViewHolder, RegardingCellRecyclerViewHolder, VacationRequestPageFragment |
| `crc644d6af55cdbf9c2ff` | `AulaNative.Droid.Activities.Calendar.InvitationFragment` | AulaNative.Droid | InvitationFragment |
| `crc6450301ba970a0a5ed` | `AulaNative.Droid.Activities.Calendar.MyCalendar` | AulaNative.Droid | BirthdayCalendarAdapter, CalendarMenuAdapter, DelegatedResourceViewHolder, EventFilterTitleViewHolder, EventFilterViewHolder, EventGoToCalendarSynchronisationRegistrations, EventGoToSearchViewHolder, MyCalendarTitleViewHolder, MyCalendarViewHolder, SingleParticipantTimeSlotAdapter |
| `crc6450e07d0e82e86181` | `Android.Gms.Common.Apis` | Xamarin.GooglePlayServices.Basement | AwaitableResultCallback`1, ResultCallback`1 |
| `crc6456b0136d0ce34f0b` | `AulaNative.Droid.Views.AulaSelectionEditor` | AulaNative.Droid | AulaSelectionActivity, AulaSelectionActivityContentViewHolder, AulaSelectionActivityHeaderViewHolder, AulaSelectionActivityLeftContentViewHolder, AulaSelectionAdapter, AulaSelectionEditor, AulaSelectionEditorNoBorder, MultipleAulaSelectionActivity, MultipleAulaSelectionAdapter, MultipleAulaSelectionAdapter+FilteringItemViewHolder, MultipleAulaSelectionEditor |
| `crc645d81b9b77a3a8305` | `AulaNative.Droid.FireBase` | AulaNative.Droid | AulaFirebaseMessagingService |
| `crc645eb5d8e01aef11fc` | `AulaNative.Droid.Activities.ActivityList` | AulaNative.Droid | ComeGoEmployeeFragment, ComeGoStaffOverviewFragment |
| `crc645f213220f61a375d` | `AulaNative.Droid.Activities.ComeGoStaff.WeekOverview.EditTimes` | AulaNative.Droid | ComeGoEmployeeWeekViewEditTimesActivity |
| `crc645f7568a2e524084e` | `AulaNative.Droid.Views.AulaCheckbox` | AulaNative.Droid | AulaCheckbox |
| `crc645f94237729b3a0f9` | `AulaNative.Droid.Activities.UserProfile.DeviceSettings` | AulaNative.Droid | FrontPageSettingViewHolder, ProfileDeviceSettingsActivity, ProfileDeviceSettingsAdapter, ProfileDeviceSettingsClearHistoryViewHolder, ProfileDeviceSettingsHeaderViewHolder, ProfileDeviceSettingsHistoryViewHolder |
| `crc645facb3ab6b15c8c1` | `AulaNative.Droid.Activities.Comments` | AulaNative.Droid | CommentCellViewHolder, CommentsActivity, CommentsAdapter, LoadMoreCommentCellViewHolder, RemovedCommentCellViewHolder |
| `crc64621e2a123352b68c` | `AulaNative.Droid.CustomViews.CommentButton` | AulaNative.Droid | CommentButton |
| `crc64629431144609bb38` | `AulaNative.Droid.Activities.Document.SecureDocuments.Overview` | AulaNative.Droid | BaseSecureDocumentOverviewAdapter, BaseSecureDocumentRecycleViewHolder, SecureDocumentBaseFragment, SecureDocumentOverviewAdapter, SecureDocumentOverviewAdapterHeaderViewHolder, SecureDocumentOverviewFilteringAdapter, SecureDocumentOverviewFilteringCell, SecureDocumentOverviewFilteringFragment, SecureDocumentOverviewFilteringSectionCell, SecureDocumentOverviewFragment, SecureDocumentOverviewSortingFragment, SecureDocumentRecycleViewHolder, SecureDocumentRemoveAssociationFragment, SecureDocumentsOverviewSelectModeActivity |
| `crc6463855a261c29f242` | `AulaNative.Droid.Activities.Document` | AulaNative.Droid | DocumentFragment |
| `crc64653b5766be8da1e1` | `AulaNative.Droid.CustomViews.AutoComplete.Abstracts` | AulaNative.Droid | BaseAulaAutoCompleteAdapter`1, BaseAulaAutoCompleteAdapter`1+BaseAulaAutoCompleteAdapterFilter, BaseAulaAutoCompleteCell, BaseAulaAutoCompleteCell`1 |
| `crc64667cb1377a348c78` | `AulaNative.Droid.Views.DateTimePicker` | AulaNative.Droid | AulaDateTimePickerEditor, DateTimePickerDialogFragment, DateTimePickerEditor, DateTimePickerLabelEditor, DateTimePickerWithClear |
| `crc6466f7fa310a49d51d` | `AulaNative.Droid.CustomViews.Editor` | AulaNative.Droid | AndroidAulaEditor, AndroidAulaEditor+AndroidAulaEditorAbsorbScrollTouchListener, AndroidAulaEditorFeatureItem, AndroidAulaMessageEditor, AttachmentSelectionListAdapter, AttachmentSelectionListViewHolder, MediaSelectionListAdapter, MediaSelectionListViewHolder, MediaTagActivity |
| `crc646887df94713985ef` | `AulaNative.Droid.Activities.Onboarding.AppOnboarding` | AulaNative.Droid | AppOnboardingFragment |
| `crc6468b6408a11370c2f` | `Microsoft.Maui.Authentication` | Microsoft.Maui.Essentials | WebAuthenticatorCallbackActivity, WebAuthenticatorIntermediateActivity |
| `crc64690f2b623932154d` | `AulaNative.Droid.CustomViews.AulaTextWebView` | AulaNative.Droid | AulaTextWebView, AulaTextWebView+AulaTextWebViewClient |
| `crc646c1bee7ebff6e2b5` | `AulaNative.Droid.CustomViews.AutoCompleteSingleSelectionControl` | AulaNative.Droid | AutoCompleteSingleSelectionControl |
| `crc646df68bf978cc5370` | `AulaNative.Droid.Activities.Document.CloudIntegration` | AulaNative.Droid | CloudIntegrationActivity |
| `crc6470a0d42cc0068d48` | `AulaNative.Droid.Activities.ActivityList.Overview` | AulaNative.Droid | ActivityItemTabletViewHolder, ActivityItemViewHolder, ActivityListAdapter, ActivityListTabletAdapter, ActivityListTabletHeaderViewHolder, BaseActivityItemViewHolder, BaseActivityListAdapter |
| `crc647178dc2a6e2144ea` | `AulaNative.Droid.Activities.ContactsList.OverView` | AulaNative.Droid | ContactListFilterAdapter, ContactListFilterItemViewHolder, ContactListFilteringGroupAdapter, ContactListFilteringGroupHeaderViewHolder, ContactListFilteringGroupInfoViewHolder, ContactListFilteringGroupSelectionViewHolder, ContactListOverviewSortingAndFilteringFragment |
| `crc64727613c41f254141` | `AulaNative.Droid.Activities.Login` | AulaNative.Droid | ForceUpdateActivity, LauncherActivity, LoginActivity, LoginWithPinActivity, OTPSelectionActivity, WebAuthenticationCallbackActivity |
| `crc64755e4274aa5e2b1a` | `AulaNative.Droid.Activities.Document.SecureDocuments.Details` | AulaNative.Droid | SecureDocumentDetailsActivity |
| `crc6479b146052ae45384` | `AulaNative.Droid.Utils.RecycleViewExtensions` | AulaNative.Droid | DimenDecoration, DimenWithSpaceDecoration |
| `crc647c21045b8bff1ef3` | `AulaNative.Droid.Activities.UserProfile.GeneralTerms` | AulaNative.Droid | ProfileGeneralTermsActivity |
| `crc647c99842bede3c88a` | `AulaNative.Droid.CustomViews.RelationsView` | AulaNative.Droid | RelationsView |
| `crc647e6651923e6daf74` | `AulaNative.Droid.Activities.ComeGo.ResponsiblePickup` | AulaNative.Droid | ComeGoPickupEditChildSwitchViewHolder, ComeGoPickupEditFormChildAdapter, ComeGoPickupResponsibleEditFormActivity |
| `crc647e759e71f16a7378` | `AulaNative.Droid.Activities.Calendar.Event` | AulaNative.Droid | BaseEventViewHolder, BirthdayCalendarFragment, CalendarLandscapeFragment, CalendarMenuActivity, CalendarPortraitAdapter, CalendarPortraitFragment, CalendarPortraitFragment+EventPortraitScrollListener, EditLessonActivity, EditMeetingActivity, EventAllDayViewHolder, EventBirthdayViewHolder, EventDetailsActivity, EventEditFormActivity, EventMonthViewHolder, EventMultiCreationLayout, EventPortraitSectionViewHolder, EventTimeSlotAdapter, EventTimeSlotAdapter+EventTimeSlotViewHolder, EventTypeActivity, EventViewHolder, MeetingParticipantSelectCellViewHolder, MeetingParticipantSelectHeaderViewHolder, MultiParticipantTimeSlotAdapter, RepeatTypeActivity, RepeatTypeRecyclerAdapter, RepeatTypeRecyclerAdapter+RepeatTypeViewHolder, RespondConversationMeetingActivity, RespondConversationMeetingParticipantSelectionActivity, SelectOtherPeopleActivity, SendEventReminderActivity, ShareCalendarActivity, SimpleSpinnerAdapter`1, TimeSlotCellContentViewHolder, TimeSlotCellHeaderViewHolder, ViewLessonActivity, ViewSchoolRecyclerViewAdapter |
| `crc648045d2d56a3c218e` | `AulaNative.Droid.Activities.Calendar.CalendarSynchronization` | AulaNative.Droid | CalendarSynchronisationActivity |
| `crc6480c34a25f88f9536` | `AulaNative.Droid.CustomViews.AulaMoreActionView` | AulaNative.Droid | AulaMoreActionView |
| `crc6482166907f5634a0d` | `AulaNative.Droid.CustomViews.OrderingView` | AulaNative.Droid | OrderingRecycleViewHolder, OrderingView, OrderingViewAdapter |
| `crc6485dd9830b50fd4a3` | `AulaNative.Droid.Activities.Calendar.Event.Notifications` | AulaNative.Droid | AllNotificationActivity |
| `crc6485f5ffda6cbe1a5a` | `AulaNative.Droid.Utils.RecyclerSwipeLayout.Implements` | AulaNative.Droid | SwipeItemMangerImpl, SwipeItemMangerImpl+OnLayoutListener, SwipeItemMangerImpl+SwipeMemory, SwipeItemMangerImpl+ValueBox, SwipeItemRecyclerMangerImpl |
| `crc6486715a2de441f74b` | `AulaNative.Droid.Activities.UserProfile.NotificationSettings` | AulaNative.Droid | ClearNotificationActivity, ClearNotificationsModulesAdapter, ModuleViewHolder, NotificationSettingsActivity, NotificationSettingsEditActivity, NotificationSettingsEditFragment, NotificationSettingsHeadlineViewHolder, NotificationSettingsItemEmailViews, NotificationSettingsItemViewHolder, NotificationSettingsListAdapter, NotificationSettingsNotAvailableViewHolder, NotificationSettingsOnboardingHeader, NotificationSettingsRadioButtonItem, NotificationSettingsRegularTextViewHolder |
| `crc6487c0dd2d74ab1a98` | `AulaNative.Droid.Activities.Onboarding.AdditionalMasterData` | AulaNative.Droid | OnboardingAdditionalMasterDataFragment |
| `crc648a510be63b920dca` | `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.LocationSectionWrapper` | AulaNative.Droid | LocationSelectionAdapter |
| `crc648b28e76e068c4131` | `AulaNative.Droid.Views.AulaValidationField` | AulaNative.Droid | AulaAdditionalDataValidationField, AulaMasterDataValidationField, AulaTextWatcher, AulaValidationErrorTextLayout |
| `crc648d07b834a19501a6` | `AulaNative.Droid.Activities.UserProfile.MasterData` | AulaNative.Droid | MasterDataActivity, ProfilePictureValidateAndRotateActivity |
| `crc648e5c3c2d7dd73a22` | `AulaNative.Droid.CustomViews` | AulaNative.Droid | AlignTopWrapperDrawable, ClickIntercepterLinearLayout, EmptyViewHolder, EventRepetitionSection, MediaActionOptionItem, NewIndicatorView, NotificationBadgeLayout, OutOfBoundCatchingGridLayoutManager, OutOfBoundCatchingLinearLayoutManager, OverlayWithTransparentCircleView, RecipientsGroupTextView, RecycleViewSectionAdapter, SectionAdapter`1, SquareLayout, ThreeStateSwitch, WarningInfoView |
| `crc648e6078e8b9fed96c` | `AulaNative.Droid.Activities.Calendar.Vacation.Details` | AulaNative.Droid | VacationDetailsActivity, VacationRegistrationChildItemViewHolder, VacationRegistrationHeaderViewHolder, VacationRegistrationOverviewActivity, VacationRegistrationOverviewAdapter, VacationRegistrationOverviewDayActivity, VacationRegistrationOverviewDayAdapter, VacationRegistrationOverviewHeaderViewHolder, VacationRegistrationOverviewItemViewHolder |
| `crc649064ebd376b6a2af` | `AulaNative.Droid.Activities.ComeGo.TimesTab.DayDetail` | AulaNative.Droid | ComeGoDetailTimesActivity |
| `crc6490c78c75c22ee2bd` | `AulaNative.Droid.Activities.Document.CommonFiles.Overview` | AulaNative.Droid | CommonFilesFilteringFragment, CommonFilesFragment, CommonFilesRecycleViewAdapter, CommonFilesRecycleViewHolder |
| `crc6492fca3f7b9721334` | `AulaNative.Droid.Activities.Album.Adapters` | AulaNative.Droid | AlbumTagAdapter, AlbumTagViewHolder, MediaSelectionAdapter |
| `crc6493855b22b6fa0721` | `Microsoft.Maui.Media` | Microsoft.Maui.Essentials | TextToSpeechInternalImplementation |
| `crc64950e68cb2d0da3b5` | `AulaNative.Droid.CustomViews.AutoCompleteControl.ResultTable` | AulaNative.Droid | AutoCompleteControlResultAdapter |
| `crc649512c951229a8649` | `AulaNative.Droid.Activities.Common.BaseActivities` | AulaNative.Droid | AulaBaseAbstractPushNotificationNavigatorActivity, AulaBaseAppCompatActivity, AulaBaseAppCompatActivity`1, AulaBaseFragment, AulaBaseFragmentActivity, AulaBaseListFragment |
| `crc64954218ca674dda46` | `AulaNative.Droid.CustomViews.GenericView` | AulaNative.Droid | GenericView |
| `crc6495d4f5d63cc5c882` | `Android.Gms.Extensions` | Xamarin.GooglePlayServices.Tasks | AwaitableTaskCompleteListener`1 |
| `crc64964578a205e3fa0b` | `AulaNative.Droid.Activities.ComeGo.TimesTab` | AulaNative.Droid | ComeGoViewTimesActivity, ComeGoViewTimesAdapter, DateHeaderViewHolder, DayScheduleViewHolder, WeekTitleViewHolder |
| `crc64982ffc6c9fa05072` | `AulaNative.Droid.Activities.CreateMessage` | AulaNative.Droid | CreateMessageActivity |
| `crc649881f3fa1611df58` | `AulaNative.Droid` | AulaNative.Droid | MainActivity, MainActivity+MainActivityOnPageChangedListener |
| `crc6499b40022eb512924` | `AulaNative.Droid.CustomViews.AutoCompleteControl.SearchRecipient` | AulaNative.Droid | SearchRecipientCellViewHolder |
| `crc649a4a06496aa5e5d8` | `AulaNative.Droid.Activities.ComeGoStaff.VacationRegistrationOverview` | AulaNative.Droid | VacationRegistrationOverviewFragment, VacationRegistrationOverviewItemViewHolder, VacationRegistrationOverviewListAdapter |
| `crc649b82ceeb99083fc1` | `AulaNative.Droid.Activities.ComeGo.RequestRegisterVacation` | AulaNative.Droid | RequestRegisterVacationActivity |
| `crc649dacca83dd48cffd` | `(default)` | AulaNative.Droid | MeetingParticipantSelectAdapter |
| `crc64a0cee6c4deaacd25` | `AulaNative.Droid.Utils.RecyclerSwipeLayout.Adapters` | AulaNative.Droid | RecyclerSwipeAdapter |
| `crc64a0fd0c3f0e48327c` | `AulaNative.Droid.CustomViews.AulaBadge` | AulaNative.Droid | AulaBadge |
| `crc64a171c606f2682d2d` | `AulaNative.Droid.Activities.ComeGo` | AulaNative.Droid | ComeGoAllNotificationActivity, ComeGoFragment, PlanningPageFragment, PlanningSubPageContainerActivity |
| `crc64a25b61d9f8ee364f` | `AndroidX.Transitions` | Xamarin.AndroidX.Transition | FloatArrayEvaluator, RectEvaluator, TransitionUtils, TransitionUtils+MatrixEvaluator |
| `crc64a53fe42ad64be0cd` | `AulaNative.Droid.Activities.Menu` | AulaNative.Droid | AbstractProfileBar, AulaBottomBarEditShortcutsView, BottomSheetAdapter`1, BottomSheetViewHolder`1, ModalBottomSheet`1, MoreMenuActivity, MoreMenuAdapter, MoreMenuViewHolder, SettingsEditShortcutsActivity, SettingsOverviewActivity |
| `crc64a670a6203b7f22cf` | `AulaNative.Droid.CustomViews.NotificationTabBadge` | AulaNative.Droid | NotificationTabBadgeLayout |
| `crc64a7dcd4496e5253be` | `AulaNative.Droid.Activities.ContactsList` | AulaNative.Droid | ContactItemEmptyViewHolder, ContactItemViewHolder, ContactListAdapter, ContactListOverviewFragment, ContactsListFragment |
| `crc64a8637d06d0024545` | `AulaNative.Droid.Activities.PersonalReferenceData.ViewOtherMasterData` | AulaNative.Droid | ViewOtherMasterDataActivity, ViewOtherMasterDataAdapter, ViewOtherMasterDataAnswerAnswerViewHolder, ViewOtherMasterDataFilterViewHolder, ViewOtherMasterDataFilteringAdapter, ViewOtherMasterDataFilteringFragment, ViewOtherMasterDataSortingFragment |
| `crc64abdb8e32447d89bd` | `AulaNative.Droid.CustomViews.Anim` | AulaNative.Droid | AnimatorListener |
| `crc64ad8b4cfa34b1cf75` | `AulaNative.Droid.Utils.BackButtonPressedHandler` | AulaNative.Droid | BackButtonPressedHandler |
| `crc64ae5416a92d00bcf3` | `AulaNative.Droid.Views.CalendarMonthView` | AulaNative.Droid | CustomCalendarViewMonth, CustomScrollCalendar, DayPhoneViewHolder, DayViewHolder, DayViewInMonthPhoneAdapter, DayViewInMonthTabletAdapter |
| `crc64af2ffd097057c735` | `AulaNative.Droid.CustomViews.CustomViewPager` | AulaNative.Droid | AulaTabLayout, SwipeDisabledViewPager |
| `crc64b07562da40b875be` | `AulaNative.Droid.Activities.Posts` | AulaNative.Droid | EditPostActivity, PostAllNotificationFragment, PostListViewWithFilterFragment, PostOverviewMultiCreationLayout, ViewPostActivity |
| `crc64b727de19c91cf26f` | `AulaNative.Droid.Activities.Posts.PostsList` | AulaNative.Droid | PostListAdapter, PostListViewHolder |
| `crc64b801ff6ba46789a9` | `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.SleepingTimeSectionWrapper` | AulaNative.Droid | AddSleepTimeViewHolder, SleepTimeContentViewHolder, SleepingTimeAdapter |
| `crc64b9870f2f6f317432` | `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.LeaveSectionWrapper` | AulaNative.Droid | LeaveTypeSelectionAdapter |
| `crc64ba438d8f48cf7e75` | `Microsoft.Maui.ApplicationModel` | Microsoft.Maui.Essentials | ActivityLifecycleContextListener, IntermediateActivity |
| `crc64ba680629999e2cd4` | `AulaNative.Droid.CustomViews.AutoComplete` | AulaNative.Droid | AulaAutoCompleteAdapter`1, AulaAutoCompleteCell, AulaAutoCompleteSectionCell, AulaAutoCompleteView, ExtendAutoCompleteTextView, FocusChangeListener |
| `crc64bb8db2cc00cd9bc0` | `AulaNative.Droid.Activities.Document.SecureDocuments.Overview.DocumentChooseMode` | AulaNative.Droid | SecureDocumentOverviewChooseModeAdapter, SecureDocumentOverviewChooseModeAdapterHeaderViewHolder, SecureDocumentOverviewChooseModeAdapterViewHolder |
| `crc64bbda72139c7d8a39` | `AulaNative.Droid.Activities.UserProfile.Consents` | AulaNative.Droid | ConsentActivity, ConsentListAdapter |
| `crc64bd0c7aa4e298a4f2` | `AulaNative.Droid.CustomViews.LinkRenderingView` | AulaNative.Droid | LinKRenderingCellViewHolder, LinkRenderingMediaPlayerActivity, LinkRenderingView, LinkRenderingViewAdapter |
| `crc64bd13f27e022481cd` | `AulaNative.Droid.Activities.Document.Overview` | AulaNative.Droid | DocumentOverviewFragment |
| `crc64bf0d10c1fe58d3f1` | `AulaNative.Droid.CustomViews.CommonViewHolders` | AulaNative.Droid | LoadingViewHolder, NoItemViewHolder, SimpleSelectableItemViewHolder |
| `crc64c0ad0b328a5409b9` | `AulaNative.Droid.Activities.Gallery.NewFolder` | AulaNative.Droid | AlbumAdapter, AlbumViewHolder, BaseAlbumViewHolder, CloudFileViewHolder, CloudFolderViewHolder, CloudIntegrationAdapter, TagFourViewHolder, TagSixViewHolder, TagTwoViewHolder |
| `crc64c1d40e40ba7282d6` | `AulaNative.Droid.Activities.Search` | AulaNative.Droid | GlobalSearchActivity, GlobalSearchTabAccessibilityDelegate, GlobalSearchViewHolder, GlobalSearchViewHolder`1, SearchAdapter, SearchCommonFileViewHolder, SearchEventViewHolder, SearchGroupViewHolder, SearchMediaViewHolder, SearchMessageViewHolder, SearchPostViewHolder, SearchProfileViewHolder, SearchSecureDocumentViewHolder |
| `crc64c23e2e7fec60456b` | `AulaNative.Droid.CustomViews.AulaPortalWebViewFragment` | AulaNative.Droid | AulaPortalWebView, AulaPortalWebView+AulaWebViewClient, AulaPortalWebViewActivity, AulaPortalWebViewFragment, WidgetWebViewFragment |
| `crc64c2bf81c26ee7219c` | `AulaNative.Droid.CustomViews.Attachment` | AulaNative.Droid | FileDowloadedBroadcastReceiver, FileViewActivity, MediaPickerActivity, ShowingFileAttachmentAdapter, ViewAttachmentViewHolder |
| `crc64c5e08b9e832e3e52` | `AulaNative.Droid.CustomViews.MoreOptions` | AulaNative.Droid | MoreOptionsCellHolder, MoreOptionsDropdownAdapter |
| `crc64c8253ed2cd0a1e08` | `AulaNative.Droid.CrossPlatformServices.AndroidEmailNotificationsHandler` | AulaNative.Droid | AndroidEmailNotificationsHandler |
| `crc64c82b547c908cd72d` | `AulaNative.Droid.Activities.Onboarding.Consent` | AulaNative.Droid | OnboardingConsentFragment |
| `crc64c83aa1b3a9eeb70d` | `AulaNative.Droid.Views.AulaSelectionEditor.SelectionInputViewWithAddMoreOption` | AulaNative.Droid | AddMoreOptionItemViewHolder, SelectionInputViewWithAddMoreOption, SelectionPageWithAddMoreOptionActivity, SelectionPageWithAddMoreOptionAdapter |
| `crc64cbdb06e5758cf86a` | `AulaNative.Droid.Activities.AdditionalMasterData.RevisionHistory` | AulaNative.Droid | AdditionalDataHistoryItemViewHolder, AdditionalDataRevisionPageActivity, AdditionalDataRevisionPageAdapter, AdditionalDataRevisionPageInstitutionProfileViewHolder |
| `crc64cd7981a54ec28d31` | `Plugin.SecureStorage` | Plugin.SecureStorage | ProtectedFileImplementation+StringKeyEntry |
| `crc64ce60827e1bb093e8` | `AulaNative.Droid.CustomViews.Gallery` | AulaNative.Droid | AulaImageView, AulaImageView+RequestListener, AulaImageZoomGestureRecognizer, AulaImageZoomView, AulaMediaControl, AulaMediaZoomControl, AulaPhotoView, AulaVideoView, BaseAulaMediaControl, GalleryViewPagerViewPager, MediaOverviewActivity, MediaOverviewActivity+ImageFragmentPagerAdapter, MediaOverviewAdapter, MediaOverviewFragment, MediaThumbnailControl, MediaThumbnailIcon, MediaViewHolder, ResourceLoadingRequestListener`1 |
| `crc64cf5f4c579eeee979` | `AulaNative.Droid.Activities.ComeGo.ResponsiblePickup.PickUpResponsiblesOverview` | AulaNative.Droid | PickUpResponsibleChildItemViewHolder, PickUpResponsiblesOverviewAdapter, PickUpResponsiblesOverviewFragment |
| `crc64d48df833906fa8fe` | `AulaNative.Droid.OAuth` | AulaNative.Droid | CloudStorageAuthInterceptor |
| `crc64d5c9e17bfc1b8195` | `AulaNative.Droid.CustomViews.OverlayRichEditorLayout` | AulaNative.Droid | OverlayRichEditorLayout, OverlayRichEditorLayout+OverlayRichEditorWebViewTouchListener |
| `crc64da28b7de7655e536` | `AulaNative.Droid.Activities.Calendar.Event.Landspace` | AulaNative.Droid | CalendarLandscapeItemFragment, CalendarLandscapeViewPagerAdapter, CustomDragRelativeLayout, LandscapeRelativeLayoutWithGrid |
| `crc64db3f50f4396b91a5` | `AulaNative.Droid.CustomViews.AndroidActionSheet` | AulaNative.Droid | ContextMenuListAdapter`1 |
| `crc64db7f9aea8cfbb131` | `AulaNative.Droid.Activities.Document.SecureDocuments.Form` | AulaNative.Droid | InternalSecureDocumentFormActivity, SecureDocumentImplicitSharingAdapter, SecureDocumentImplicitSharingViewHolder, SecureDocumentShareWithFormActivity |
| `crc64de339c188f2b9079` | `AulaNative.Droid.CustomViews.AutoCompleteSelectionList` | AulaNative.Droid | AulaAutoCompleteRecyclerViewItemMarginDecoration, AulaAutoCompleteSelectionListAdapter`1, AulaAutoCompleteSelectionListTextView, AulaAutoCompleteSelectionListViewHolder`1, BaseAulaAutoCompleteSelectionListResultAdapter`1, BaseAulaAutoCompleteSelectionListResultViewHolder, BaseAulaAutoCompleteSelectionListResultViewHolder`1 |
| `crc64def7a5631a95e5c8` | `AulaNative.Droid.Activities.ComeGo.DailyOverviewTab` | AulaNative.Droid | ComeGoPickUpInfoFormActivity, DailyOverviewAdapter, DailyOverviewItemViewHolder, SpareTimeActivityFormActivity |
| `crc64df89d32432429ade` | `AulaNative.Droid.Activities.Calendar.CalendarSynchronisation` | AulaNative.Droid | CalendarSynchronisationCreationActivity, CalendarSynchronisationCreationAdapter, CalendarSynchronisationDetailedOverviewActivity, CalendarSynchronisationOverviewActivity, CalendarSynchronisationOverviewAdapter, CheckboxViewHolder, RecyclerViewHolder |
| `crc64e01ba431b1ef99e4` | `AulaNative.Droid.Views.AulaImageAbbreviation` | AulaNative.Droid | AulaImageAbbreviation, AulaImageAbbreviation+ImageRequestListener, FaceCenteredCropBitmapTransformation |
| `crc64e16ab6386d88b6e5` | `AulaNative.Droid.Activities.Gallery.Adapters` | AulaNative.Droid | AlbumDetailsFragment, AlbumDetailsFragment+BackButtonKeyListener, AlbumSelectedAdapter, AlbumTagAutoCompleteViewHolder, MediaAdapter, MediaImportFragment, MediaViewHeaderHolder, MediaViewHolder, SelectUserToTagAdapter, UserToTagViewHolder |
| `crc64e3bc9524bf401dcf` | `AulaNative.Droid.Activities.UserProfile` | AulaNative.Droid | ProfileActivity, ProfileAdapter, ProfileGridAdapter, ProfileImageViewHolder, ProfileOptionViewHolder, ProfilePrivacyAndLogOutViewHolder, ProfileRelationsCellViewHolder |
| `crc64e53d2f592022988e` | `Microsoft.Maui.Networking` | Microsoft.Maui.Essentials | ConnectivityBroadcastReceiver, ConnectivityImplementation+EssentialsNetworkCallback |
| `crc64e5bd511b672d6dc6` | `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.OutOfSchoolSectionWrapper` | AulaNative.Droid | OutOfSchoolSelectionAdapter |
| `crc64e63ddd64dd3402b1` | `AulaNative.Droid.Activities` | AulaNative.Droid | TestingActivity |
| `crc64e6d0b84c6264ccdd` | `Plugin.Fingerprint` | Plugin.Fingerprint | AuthenticationHandler |
| `crc64e6f1f86f3a60c4db` | `AulaNative.Droid.CustomListeners` | AulaNative.Droid | DirectionTouchListener, ScrollDownDetector, ScrollStoppedDetector |
| `crc64e7f547c073373627` | `AulaNative.Droid.Activities.Calendar.Vacation` | AulaNative.Droid | EditVacationRequestActivity, VacationRegistrationItemViewHolder, VacationRegistrationOverviewListAdapter |
| `crc64e9e062cc12d35878` | `AulaNative.Droid.Activities.Messages.AutoReply` | AulaNative.Droid | AutoReplyActivity |
| `crc64ea34368975947079` | `AulaNative.Droid.Activities.Common.GenericProfileList` | AulaNative.Droid | GenericProfileListItemViewHolder, GenericProfileListSectionViewHolder |
| `crc64ea6dad1577ee648b` | `AulaNative.Droid.CustomViews.SearchRecipient` | AulaNative.Droid | BaseSearchRecipientAutoCompleteAdapter`1, SearchRecipientAutoCompleteAdapter, SearchRecipientAutoCompleteCell |
| `crc64eda3ac7f2ee510d4` | `AulaNative.Droid.Activities.AdditionalMasterData` | AulaNative.Droid | AdditionalMasterDataInstitutionProfileView, AdditionalMasterDataSingleResponseView, ProfileAdditionalMasterdataActivity |
| `crc64ee5c41ee04c846c1` | `AulaNative.Droid.Activities.Overview` | AulaNative.Droid | OverviewFragment, PostsAllNotificationActivity |
| `crc64ef1cca385a178d29` | `AulaNative.Droid.Activities.Common` | AulaNative.Droid | AulaFragmentViewPagerAdapter, AulaNaivgationRootFragment, AulaNavigationFragment, EmptyFragment, NoDataViewHolder, ReportActivity |
| `crc64f0703b11dec00ad7` | `AulaNative.Droid.Activities.ComeGoStaff.Overview` | AulaNative.Droid | ActivityListOverviewFragment |
| `crc64f181849ee93de040` | `AulaNative.Droid.Plugins.RequestPermission` | AulaNative.Droid | RequestPermissionActivity |
| `crc64f21d13d69ef5a942` | `AulaNative.Droid.Activities.Document.SecureDocuments.CustomViews.DocumentRecipient` | AulaNative.Droid | DocumentRecipientAutoCompleteResultViewHolder |
| `crc64f2e6333666238999` | `AulaNative.Droid.Activities.Groups.GroupMembershipsList` | AulaNative.Droid | GroupMembershipsAdapter, GroupMembershipsCellViewHolder, GroupMembershipsSectionViewHolder |
| `crc64f408b88a4e473f92` | `AulaNative.Droid.Activities.ComeGo.AbsenceTab.ViewHolders` | AulaNative.Droid | AbsenceBaseHeaderViewHolder, AbsenceChildSubHeaderCellViewHolder, AbsenceNoDataCellViewHolder, AbsenceTabContainerRecyclerViewAdapter, AbsenceVacationCellViewHolder, AbsenceVacationHeaderButtonViewHolder, AbsenceVacationRequestCellViewHolder |
| `crc64f48a125ae4e10cda` | `AulaNative.Droid.Activities.Gallery` | AulaNative.Droid | AlbumDetailsActivity, AlbumSelectedActivity, GalleryFragment, GenericRecipientsListAdapter, GridViewSpanSizeLookup, MediaTaggingPageActivity, RecipientsViewActivity, SelectUserToTagActivity |
| `crc64f62664462a8937a9` | `Microsoft.Maui.Devices.Sensors` | Microsoft.Maui.Essentials | AccelerometerListener, BarometerListener, ContinuousLocationListener, GyroscopeListener, MagnetometerListener, OrientationSensorListener, SensorListener, SingleLocationListener |
| `crc64f723dd1a15c408a5` | `AulaNative.Droid.Activities.Partials` | AulaNative.Droid | AddRecipientsDialog, AulaToast, CalendarNotificationAdapter, CalendarNotificationViewHolder, CustomDialog, CustomListAdapter, EmptyViewHolder, EventViewHolder, SearchHighlightedWebView, TopNotificationListAdapter, TopNotificationViewHolder |
| `crc64f75412af3a986ba9` | `AulaNative.Droid.CustomViews.AlphabetFastScrollView` | AulaNative.Droid | AlphabletScrollerAdapter, CharacterViewHolder, RecyclerViewFastScroller, RecyclerViewFastScroller+BubbleEventListener |
| `crc64f89139c0fb2fd8a0` | `AulaNative.Droid.Activities.UserProfile.Consents.ViewHolders` | AulaNative.Droid | ConsentListViewHolder, OnboardingTextsViewHolder, SingleConsentEditHolder, SingleConsentViewHolder |
| `crc64f8d3246b1f4d1c76` | `AulaNative.Droid.Activities.ComeGo.Overview` | AulaNative.Droid | ComeGoDailyOverviewTabFragment, ComeGoGuardianOverviewFragment |
| `crc64fbe4ecea5fe75301` | `AulaNative.Droid.Activities.ComeGo.ForcedRegisterVacation` | AulaNative.Droid | ForcedRegisterVacationAdapter, ForcedRegisterVacationAdapter+ForcedRegisterVacationCloseDayViewHolder, ForcedRegisterVacationAdapter+ForcedRegisterVacationContentViewHolder, ForcedRegisterVacationAdapter+ForcedRegisterVacationHeaderViewHolder, GuardianRegisterVacationOnEmployeeRequestActivity |
| `crc64fe3874eccf51a10b` | `AulaNative.Droid.Activities.ComeGo.OpeningHoursAndClosedDays` | AulaNative.Droid | ComeGoOpeningAndClosedHoursInstitutionAdapter, ComeGoOpeningHoursAndClosedDaysContentCellViewHolder, ComeGoOpeningHoursAndClosedDaysHeaderCellViewHolder, ComeGoOpeningHoursAndClosedDaysInstitutionListFragment, ComeGoOpeningHoursAndClosedDaysInstitutionViewHolder, ComeGoOpeningHoursAndClosedDaysOverviewPageActivity, ComeGoOpeningHoursAndClosedDaysOverviewPageAdapter, ComeGoOpeningHoursAndClosedDaysOverviewPageFragment |
| `crc64fe884fa183cc4ca0` | `AulaNative.Droid.Activities.ComeGoStaff.Overview.Filter.GeneralFiltering` | AulaNative.Droid | ActivityListGeneralFilteringActivity, ActivityListGeneralFilteringAdapter, ActivityListGeneralFilteringAdapter+GeneralFilteringCheckboxViewHolder, ActivityListGeneralFilteringAdapter+GeneralFilteringHeader, ActivityListGeneralFilteringAdapter+GeneralFilteringRadioButtonViewHolder, ActivityListGeneralFilteringAdapter+GeneralFilteringViewHolder |

## Namespace Groups by Functional Domain


### Activities (School)

- **`crc645eb5d8e01aef11fc`** -> `AulaNative.Droid.Activities.ActivityList`
  - ComeGoEmployeeFragment
  - ComeGoStaffOverviewFragment
- **`crc6470a0d42cc0068d48`** -> `AulaNative.Droid.Activities.ActivityList.Overview`
  - ActivityItemTabletViewHolder
  - ActivityItemViewHolder
  - ActivityListAdapter
  - ActivityListTabletAdapter
  - ActivityListTabletHeaderViewHolder
  - BaseActivityItemViewHolder
  - BaseActivityListAdapter

### Albums & Media

- **`crc640d97cec38b77325f`** -> `AulaNative.Droid.Activities.Album`
  - AlbumEditFormActivity
  - AlbumEditFormFragment
  - AlbumTagFragment
  - FilePickerActivity
  - GalleryPickerActivity
  - ImportMediaContainerActivity
  - MediaSelectionFragment
  - MediaTagFragment
- **`crc6422b55094979017ec`** -> `AulaNative.Droid.CustomViews.FullscreenMediaPlayer`
  - FullscreenMediaPlayerActivity
  - FullscreenMediaPlayerActivity+OnInfoListener
  - FullscreenMediaPlayerActivity+OnPreparedListener
- **`crc642856e7fed4fd87c9`** -> `AulaNative.Droid.Activities.Album.ViewHolders`
  - BaseViewHolder
  - MediaSelectionOtherItemViewHolder
- **`crc6492fca3f7b9721334`** -> `AulaNative.Droid.Activities.Album.Adapters`
  - AlbumTagAdapter
  - AlbumTagViewHolder
  - MediaSelectionAdapter
- **`crc6493855b22b6fa0721`** -> `Microsoft.Maui.Media`
  - TextToSpeechInternalImplementation
- **`crc64c0ad0b328a5409b9`** -> `AulaNative.Droid.Activities.Gallery.NewFolder`
  - AlbumAdapter
  - AlbumViewHolder
  - BaseAlbumViewHolder
  - CloudFileViewHolder
  - CloudFolderViewHolder
  - CloudIntegrationAdapter
  - TagFourViewHolder
  - TagSixViewHolder
  - TagTwoViewHolder
- **`crc64ce60827e1bb093e8`** -> `AulaNative.Droid.CustomViews.Gallery`
  - AulaImageView
  - AulaImageView+RequestListener
  - AulaImageZoomGestureRecognizer
  - AulaImageZoomView
  - AulaMediaControl
  - AulaMediaZoomControl
  - AulaPhotoView
  - AulaVideoView
  - BaseAulaMediaControl
  - GalleryViewPagerViewPager
  - MediaOverviewActivity
  - MediaOverviewActivity+ImageFragmentPagerAdapter
  - MediaOverviewAdapter
  - MediaOverviewFragment
  - MediaThumbnailControl
  - MediaThumbnailIcon
  - MediaViewHolder
  - ResourceLoadingRequestListener`1
- **`crc64e16ab6386d88b6e5`** -> `AulaNative.Droid.Activities.Gallery.Adapters`
  - AlbumDetailsFragment
  - AlbumDetailsFragment+BackButtonKeyListener
  - AlbumSelectedAdapter
  - AlbumTagAutoCompleteViewHolder
  - MediaAdapter
  - MediaImportFragment
  - MediaViewHeaderHolder
  - MediaViewHolder
  - SelectUserToTagAdapter
  - UserToTagViewHolder
- **`crc64f48a125ae4e10cda`** -> `AulaNative.Droid.Activities.Gallery`
  - AlbumDetailsActivity
  - AlbumSelectedActivity
  - GalleryFragment
  - GenericRecipientsListAdapter
  - GridViewSpanSizeLookup
  - MediaTaggingPageActivity
  - RecipientsViewActivity
  - SelectUserToTagActivity

### Authentication & Login

- **`crc6468b6408a11370c2f`** -> `Microsoft.Maui.Authentication`
  - WebAuthenticatorCallbackActivity
  - WebAuthenticatorIntermediateActivity
- **`crc64727613c41f254141`** -> `AulaNative.Droid.Activities.Login`
  - ForceUpdateActivity
  - LauncherActivity
  - LoginActivity
  - LoginWithPinActivity
  - OTPSelectionActivity
  - WebAuthenticationCallbackActivity
- **`crc64d48df833906fa8fe`** -> `AulaNative.Droid.OAuth`
  - CloudStorageAuthInterceptor

### Base & Infrastructure

- **`crc641b0203710e4b7ee0`** -> `AulaNative.Droid.ViewModels.Common`
  - DetailsTabBarItem`1
  - TabBarItem
- **`crc6422aa70c17a2f84a5`** -> `AulaNative.Droid.Utils.RecyclerSwipeLayout.Util`
  - Attributes
- **`crc642a61f038ea047fcc`** -> `AulaNative.Droid.Utils.Decorators`
  - DividerWithoutFirstAndLastLineItemDecorator
- **`crc6434af9c19aa01b597`** -> `Android.Gms.Common.Apis`
  - GoogleApiClientConnectionCallbacksImpl
  - GoogleApiClientOnConnectionFailedListenerImpl
- **`crc6436b99b97cc67a1cc`** -> `AulaNative.Droid.Utils`
  - AccessibilityDelegate`1
  - AccessibilityUtils+AccessibilityValueDelegate
  - ActivityResultCallback
  - BasicHeaderViewHolder
  - DrawableExtension+ScaledDrawableWrapper
  - LinkClickableSpan
  - RecyclerEventViewOnScrollListener
  - RecyclerViewOnScrollListener
  - ViewTreeObserverUtils+TemporaryGlobalLayoutListener
- **`crc6450e07d0e82e86181`** -> `Android.Gms.Common.Apis`
  - AwaitableResultCallback`1
  - ResultCallback`1
- **`crc6479b146052ae45384`** -> `AulaNative.Droid.Utils.RecycleViewExtensions`
  - DimenDecoration
  - DimenWithSpaceDecoration
- **`crc6485f5ffda6cbe1a5a`** -> `AulaNative.Droid.Utils.RecyclerSwipeLayout.Implements`
  - SwipeItemMangerImpl
  - SwipeItemMangerImpl+OnLayoutListener
  - SwipeItemMangerImpl+SwipeMemory
  - SwipeItemMangerImpl+ValueBox
  - SwipeItemRecyclerMangerImpl
- **`crc649512c951229a8649`** -> `AulaNative.Droid.Activities.Common.BaseActivities`
  - AulaBaseAbstractPushNotificationNavigatorActivity
  - AulaBaseAppCompatActivity
  - AulaBaseAppCompatActivity`1
  - AulaBaseFragment
  - AulaBaseFragmentActivity
  - AulaBaseListFragment
- **`crc64a0cee6c4deaacd25`** -> `AulaNative.Droid.Utils.RecyclerSwipeLayout.Adapters`
  - RecyclerSwipeAdapter
- **`crc64ad8b4cfa34b1cf75`** -> `AulaNative.Droid.Utils.BackButtonPressedHandler`
  - BackButtonPressedHandler
- **`crc64ef1cca385a178d29`** -> `AulaNative.Droid.Activities.Common`
  - AulaFragmentViewPagerAdapter
  - AulaNaivgationRootFragment
  - AulaNavigationFragment
  - EmptyFragment
  - NoDataViewHolder
  - ReportActivity
- **`crc64f181849ee93de040`** -> `AulaNative.Droid.Plugins.RequestPermission`
  - RequestPermissionActivity

### Calendar & Events

- **`crc640c5494012a06d265`** -> `AulaNative.Droid.Activities.Calendar.Event.Overlapping`
  - EventViewHolder
  - OverLappingAdapter
  - OverlappingActivity
  - OverlappingHeaderViewHolder
- **`crc642b71a4216b6636ac`** -> `AulaNative.Droid.Views.CalendarView`
  - EventTextView
- **`crc6433c817910c30379b`** -> `AulaNative.Droid.Activities.Calendar.ImportantDate`
  - ImportantDateAdapter
  - ImportantDateFragment
  - ImportantDateViewHolder
  - SpaceItemDecoration
- **`crc64464655eca3cd0aac`** -> `AulaNative.Droid.Activities.Calendar`
  - CalendarFragment
  - CalendarOverviewFragment
  - CalendarViewPagerAdapter
- **`crc644d6af55cdbf9c2ff`** -> `AulaNative.Droid.Activities.Calendar.InvitationFragment`
  - InvitationFragment
- **`crc6450301ba970a0a5ed`** -> `AulaNative.Droid.Activities.Calendar.MyCalendar`
  - BirthdayCalendarAdapter
  - CalendarMenuAdapter
  - DelegatedResourceViewHolder
  - EventFilterTitleViewHolder
  - EventFilterViewHolder
  - EventGoToCalendarSynchronisationRegistrations
  - EventGoToSearchViewHolder
  - MyCalendarTitleViewHolder
  - MyCalendarViewHolder
  - SingleParticipantTimeSlotAdapter
- **`crc647e759e71f16a7378`** -> `AulaNative.Droid.Activities.Calendar.Event`
  - BaseEventViewHolder
  - BirthdayCalendarFragment
  - CalendarLandscapeFragment
  - CalendarMenuActivity
  - CalendarPortraitAdapter
  - CalendarPortraitFragment
  - CalendarPortraitFragment+EventPortraitScrollListener
  - EditLessonActivity
  - EditMeetingActivity
  - EventAllDayViewHolder
  - EventBirthdayViewHolder
  - EventDetailsActivity
  - EventEditFormActivity
  - EventMonthViewHolder
  - EventMultiCreationLayout
  - EventPortraitSectionViewHolder
  - EventTimeSlotAdapter
  - EventTimeSlotAdapter+EventTimeSlotViewHolder
  - EventTypeActivity
  - EventViewHolder
  - MeetingParticipantSelectCellViewHolder
  - MeetingParticipantSelectHeaderViewHolder
  - MultiParticipantTimeSlotAdapter
  - RepeatTypeActivity
  - RepeatTypeRecyclerAdapter
  - RepeatTypeRecyclerAdapter+RepeatTypeViewHolder
  - RespondConversationMeetingActivity
  - RespondConversationMeetingParticipantSelectionActivity
  - SelectOtherPeopleActivity
  - SendEventReminderActivity
  - ShareCalendarActivity
  - SimpleSpinnerAdapter`1
  - TimeSlotCellContentViewHolder
  - TimeSlotCellHeaderViewHolder
  - ViewLessonActivity
  - ViewSchoolRecyclerViewAdapter
- **`crc648045d2d56a3c218e`** -> `AulaNative.Droid.Activities.Calendar.CalendarSynchronization`
  - CalendarSynchronisationActivity
- **`crc6485dd9830b50fd4a3`** -> `AulaNative.Droid.Activities.Calendar.Event.Notifications`
  - AllNotificationActivity
- **`crc648e6078e8b9fed96c`** -> `AulaNative.Droid.Activities.Calendar.Vacation.Details`
  - VacationDetailsActivity
  - VacationRegistrationChildItemViewHolder
  - VacationRegistrationHeaderViewHolder
  - VacationRegistrationOverviewActivity
  - VacationRegistrationOverviewAdapter
  - VacationRegistrationOverviewDayActivity
  - VacationRegistrationOverviewDayAdapter
  - VacationRegistrationOverviewHeaderViewHolder
  - VacationRegistrationOverviewItemViewHolder
- **`crc64ae5416a92d00bcf3`** -> `AulaNative.Droid.Views.CalendarMonthView`
  - CustomCalendarViewMonth
  - CustomScrollCalendar
  - DayPhoneViewHolder
  - DayViewHolder
  - DayViewInMonthPhoneAdapter
  - DayViewInMonthTabletAdapter
- **`crc64da28b7de7655e536`** -> `AulaNative.Droid.Activities.Calendar.Event.Landspace`
  - CalendarLandscapeItemFragment
  - CalendarLandscapeViewPagerAdapter
  - CustomDragRelativeLayout
  - LandscapeRelativeLayoutWithGrid
- **`crc64df89d32432429ade`** -> `AulaNative.Droid.Activities.Calendar.CalendarSynchronisation`
  - CalendarSynchronisationCreationActivity
  - CalendarSynchronisationCreationAdapter
  - CalendarSynchronisationDetailedOverviewActivity
  - CalendarSynchronisationOverviewActivity
  - CalendarSynchronisationOverviewAdapter
  - CheckboxViewHolder
  - RecyclerViewHolder
- **`crc64e7f547c073373627`** -> `AulaNative.Droid.Activities.Calendar.Vacation`
  - EditVacationRequestActivity
  - VacationRegistrationItemViewHolder
  - VacationRegistrationOverviewListAdapter

### Come & Go (Attendance)

- **`crc641e4a654bb7dceae5`** -> `AulaNative.Droid.Activities.ComeGoStaff.WeekOverview.PresenceChildren`
  - PresenceChildrenDistributionActivity
  - PresenceChildrenDistributionActivityAdapter
  - PresenceChildrenDistributionActivityAdapter+ContentViewHolder
  - PresenceChildrenDistributionActivityAdapter+HeaderViewHolder
- **`crc6428737b50d58965aa`** -> `AulaNative.Droid.Activities.ComeGoStaff.WeekOverview`
  - NoDataWeekOverviewItemViewHolder
  - PhoneWeekOverviewItemViewHolder
  - TabletWeekOverviewHeaderItemViewHolder
  - TabletWeekOverviewItemViewHolder
  - WeekOverviewFragment
  - WeekOverviewPhoneListAdapter
  - WeekOverviewTabletListAdapter
- **`crc642b5ea7562757e68e`** -> `AulaNative.Droid.Activities.ComeGoStaff.Overview.Filter.DepartmentAndGroupsFiltering`
  - ActivityListDepartmentAndGroupsFilteringActivity
  - ActivityListDepartmentAndGroupsFilteringActivityAdapter
  - ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringContentViewHolder
  - ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringDepartmentViewHolder
  - ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringGroupViewHolder
  - ActivityListDepartmentAndGroupsFilteringActivityAdapter+FilteringHeaderViewHolder
- **`crc642db8434b2db369d4`** -> `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.ProfileSectionWrapper`
  - MultipleProfilesSectionWrapper+ComeGoMultipleProfileChildViewHolder
  - MultipleProfilesSectionWrapper+MultipleChildrenAdapter
- **`crc643ccd8ae99eb00e6c`** -> `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm`
  - ComeGoActivityEditFormActivity
  - ComeGoGenericSelectionAdapter`1
  - ComeGoGenericSelectionAdapter`1+GenericSelectionViewHolder
- **`crc64439a32af6af2bcba`** -> `AulaNative.Droid.Activities.ComeGoStaff.VacationList`
  - VacationListOverviewFragment
- **`crc644caf61526f1f2e48`** -> `AulaNative.Droid.Activities.ComeGo.AbsenceTab`
  - AbsenceVacationActivity
  - AbsenceVacationRecyclerViewAdapter
  - ChildSwitchCellRecyclerViewHolder
  - ContentCellRecyclerViewHolder
  - RegardingCellRecyclerViewHolder
  - VacationRequestPageFragment
- **`crc645f213220f61a375d`** -> `AulaNative.Droid.Activities.ComeGoStaff.WeekOverview.EditTimes`
  - ComeGoEmployeeWeekViewEditTimesActivity
- **`crc647e6651923e6daf74`** -> `AulaNative.Droid.Activities.ComeGo.ResponsiblePickup`
  - ComeGoPickupEditChildSwitchViewHolder
  - ComeGoPickupEditFormChildAdapter
  - ComeGoPickupResponsibleEditFormActivity
- **`crc648a510be63b920dca`** -> `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.LocationSectionWrapper`
  - LocationSelectionAdapter
- **`crc649064ebd376b6a2af`** -> `AulaNative.Droid.Activities.ComeGo.TimesTab.DayDetail`
  - ComeGoDetailTimesActivity
- **`crc64964578a205e3fa0b`** -> `AulaNative.Droid.Activities.ComeGo.TimesTab`
  - ComeGoViewTimesActivity
  - ComeGoViewTimesAdapter
  - DateHeaderViewHolder
  - DayScheduleViewHolder
  - WeekTitleViewHolder
- **`crc649a4a06496aa5e5d8`** -> `AulaNative.Droid.Activities.ComeGoStaff.VacationRegistrationOverview`
  - VacationRegistrationOverviewFragment
  - VacationRegistrationOverviewItemViewHolder
  - VacationRegistrationOverviewListAdapter
- **`crc649b82ceeb99083fc1`** -> `AulaNative.Droid.Activities.ComeGo.RequestRegisterVacation`
  - RequestRegisterVacationActivity
- **`crc64a171c606f2682d2d`** -> `AulaNative.Droid.Activities.ComeGo`
  - ComeGoAllNotificationActivity
  - ComeGoFragment
  - PlanningPageFragment
  - PlanningSubPageContainerActivity
- **`crc64b801ff6ba46789a9`** -> `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.SleepingTimeSectionWrapper`
  - AddSleepTimeViewHolder
  - SleepTimeContentViewHolder
  - SleepingTimeAdapter
- **`crc64b9870f2f6f317432`** -> `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.LeaveSectionWrapper`
  - LeaveTypeSelectionAdapter
- **`crc64cf5f4c579eeee979`** -> `AulaNative.Droid.Activities.ComeGo.ResponsiblePickup.PickUpResponsiblesOverview`
  - PickUpResponsibleChildItemViewHolder
  - PickUpResponsiblesOverviewAdapter
  - PickUpResponsiblesOverviewFragment
- **`crc64def7a5631a95e5c8`** -> `AulaNative.Droid.Activities.ComeGo.DailyOverviewTab`
  - ComeGoPickUpInfoFormActivity
  - DailyOverviewAdapter
  - DailyOverviewItemViewHolder
  - SpareTimeActivityFormActivity
- **`crc64e5bd511b672d6dc6`** -> `AulaNative.Droid.Activities.ComeGoStaff.ActivityEditForm.Sections.OutOfSchoolSectionWrapper`
  - OutOfSchoolSelectionAdapter
- **`crc64f0703b11dec00ad7`** -> `AulaNative.Droid.Activities.ComeGoStaff.Overview`
  - ActivityListOverviewFragment
- **`crc64f408b88a4e473f92`** -> `AulaNative.Droid.Activities.ComeGo.AbsenceTab.ViewHolders`
  - AbsenceBaseHeaderViewHolder
  - AbsenceChildSubHeaderCellViewHolder
  - AbsenceNoDataCellViewHolder
  - AbsenceTabContainerRecyclerViewAdapter
  - AbsenceVacationCellViewHolder
  - AbsenceVacationHeaderButtonViewHolder
  - AbsenceVacationRequestCellViewHolder
- **`crc64f8d3246b1f4d1c76`** -> `AulaNative.Droid.Activities.ComeGo.Overview`
  - ComeGoDailyOverviewTabFragment
  - ComeGoGuardianOverviewFragment
- **`crc64fbe4ecea5fe75301`** -> `AulaNative.Droid.Activities.ComeGo.ForcedRegisterVacation`
  - ForcedRegisterVacationAdapter
  - ForcedRegisterVacationAdapter+ForcedRegisterVacationCloseDayViewHolder
  - ForcedRegisterVacationAdapter+ForcedRegisterVacationContentViewHolder
  - ForcedRegisterVacationAdapter+ForcedRegisterVacationHeaderViewHolder
  - GuardianRegisterVacationOnEmployeeRequestActivity
- **`crc64fe3874eccf51a10b`** -> `AulaNative.Droid.Activities.ComeGo.OpeningHoursAndClosedDays`
  - ComeGoOpeningAndClosedHoursInstitutionAdapter
  - ComeGoOpeningHoursAndClosedDaysContentCellViewHolder
  - ComeGoOpeningHoursAndClosedDaysHeaderCellViewHolder
  - ComeGoOpeningHoursAndClosedDaysInstitutionListFragment
  - ComeGoOpeningHoursAndClosedDaysInstitutionViewHolder
  - ComeGoOpeningHoursAndClosedDaysOverviewPageActivity
  - ComeGoOpeningHoursAndClosedDaysOverviewPageAdapter
  - ComeGoOpeningHoursAndClosedDaysOverviewPageFragment
- **`crc64fe884fa183cc4ca0`** -> `AulaNative.Droid.Activities.ComeGoStaff.Overview.Filter.GeneralFiltering`
  - ActivityListGeneralFilteringActivity
  - ActivityListGeneralFilteringAdapter
  - ActivityListGeneralFilteringAdapter+GeneralFilteringCheckboxViewHolder
  - ActivityListGeneralFilteringAdapter+GeneralFilteringHeader
  - ActivityListGeneralFilteringAdapter+GeneralFilteringRadioButtonViewHolder
  - ActivityListGeneralFilteringAdapter+GeneralFilteringViewHolder

### Comments

- **`crc645facb3ab6b15c8c1`** -> `AulaNative.Droid.Activities.Comments`
  - CommentCellViewHolder
  - CommentsActivity
  - CommentsAdapter
  - LoadMoreCommentCellViewHolder
  - RemovedCommentCellViewHolder
- **`crc64621e2a123352b68c`** -> `AulaNative.Droid.CustomViews.CommentButton`
  - CommentButton

### Custom Views & UI

- **`crc640056e568e07a5371`** -> `AulaNative.Droid.CustomViews.AulaSwitch`
  - AulaSwitch
- **`crc6409d51c9fa7ea23bb`** -> `AulaNative.Droid.CustomViews.AulaTextView`
  - AulaButton
  - AulaTextView
  - AulaTextViewWithArrowLinearLayout
- **`crc640c05e18e2e9d438c`** -> `AulaNative.Droid.Views.AulaStatusImage`
  - AulaStatusImage
- **`crc640c25afc07e567a01`** -> `AulaNative.Droid.CustomViews.ListModalBottomSheet`
  - ListModalBottomSheet
  - ListModalBottomSheetAdapter
  - ListModalBottomSheetTwoLinesViewHolder
  - ListModalBottomSheetViewHolder
- **`crc640e895d78a388fb08`** -> `AulaNative.Droid.Views.MainPage`
  - AulaBottomBarEditShortcutsItemView
  - AulaBottomNavigationItemView
  - AulaMainPageBottomNavigationView
- **`crc6410c8e6ba9ee100a4`** -> `AulaNative.Droid.Views.MultiAulaImageAbbreviation`
  - MultiAulaImageAbbreviation
- **`crc6412763d6b05397a23`** -> `AulaNative.Droid.Views`
  - AulaSearchBar
- **`crc64153da6e621868364`** -> `AulaNative.Droid.CustomViews.AulaFilterAndSortView`
  - AulaFilterAndSortDropdown
  - FilterAndSortCellHolder
  - FilterAndSortDropdownAdapter
- **`crc6421c919819d958882`** -> `AulaNative.Droid.CustomViews.AutoCompleteControl.SelectionPage`
  - AutoCompleteControlSelectionPageActivity
  - AutoCompleteControlSelectionPageActivity+SelectionPageScrollListener
  - AutoCompleteControlSelectionPageAdapter
  - AutocompleteControlSelectionPageCellViewHolder
  - AutocompleteControlSelectionPageCellViewHolder`1
  - AutocompleteControlSelectionPageSectionViewHolder
- **`crc64276fbe2dc15076e4`** -> `AulaNative.Droid.CustomViews.AutoCompleteControl`
  - AutoCompleteControlEditor
- **`crc64379f1df1f1ef6185`** -> `AulaNative.Droid.Views.SelectionListView`
  - AulaSelectionHeaderViewHolder
  - AulaSelectionViewHolder
  - SelectionListAdapter
  - SelectionListView
- **`crc64427d32b40b56664a`** -> `AulaNative.Droid.CustomViews.RoundedCornerButton`
  - RoundedCornerButton
- **`crc64460bdd3cfa678830`** -> `AulaNative.Droid.CustomViews.TooltipButton`
  - TooltipButton
- **`crc64465cc550138d14d3`** -> `AulaNative.Droid.Views.LoadingOverlay`
  - LoadingOverlay
- **`crc6456b0136d0ce34f0b`** -> `AulaNative.Droid.Views.AulaSelectionEditor`
  - AulaSelectionActivity
  - AulaSelectionActivityContentViewHolder
  - AulaSelectionActivityHeaderViewHolder
  - AulaSelectionActivityLeftContentViewHolder
  - AulaSelectionAdapter
  - AulaSelectionEditor
  - AulaSelectionEditorNoBorder
  - MultipleAulaSelectionActivity
  - MultipleAulaSelectionAdapter
  - MultipleAulaSelectionAdapter+FilteringItemViewHolder
  - MultipleAulaSelectionEditor
- **`crc645f7568a2e524084e`** -> `AulaNative.Droid.Views.AulaCheckbox`
  - AulaCheckbox
- **`crc64653b5766be8da1e1`** -> `AulaNative.Droid.CustomViews.AutoComplete.Abstracts`
  - BaseAulaAutoCompleteAdapter`1
  - BaseAulaAutoCompleteAdapter`1+BaseAulaAutoCompleteAdapterFilter
  - BaseAulaAutoCompleteCell
  - BaseAulaAutoCompleteCell`1
- **`crc64667cb1377a348c78`** -> `AulaNative.Droid.Views.DateTimePicker`
  - AulaDateTimePickerEditor
  - DateTimePickerDialogFragment
  - DateTimePickerEditor
  - DateTimePickerLabelEditor
  - DateTimePickerWithClear
- **`crc6466f7fa310a49d51d`** -> `AulaNative.Droid.CustomViews.Editor`
  - AndroidAulaEditor
  - AndroidAulaEditor+AndroidAulaEditorAbsorbScrollTouchListener
  - AndroidAulaEditorFeatureItem
  - AndroidAulaMessageEditor
  - AttachmentSelectionListAdapter
  - AttachmentSelectionListViewHolder
  - MediaSelectionListAdapter
  - MediaSelectionListViewHolder
  - MediaTagActivity
- **`crc64690f2b623932154d`** -> `AulaNative.Droid.CustomViews.AulaTextWebView`
  - AulaTextWebView
  - AulaTextWebView+AulaTextWebViewClient
- **`crc646c1bee7ebff6e2b5`** -> `AulaNative.Droid.CustomViews.AutoCompleteSingleSelectionControl`
  - AutoCompleteSingleSelectionControl
- **`crc647c99842bede3c88a`** -> `AulaNative.Droid.CustomViews.RelationsView`
  - RelationsView
- **`crc6480c34a25f88f9536`** -> `AulaNative.Droid.CustomViews.AulaMoreActionView`
  - AulaMoreActionView
- **`crc6482166907f5634a0d`** -> `AulaNative.Droid.CustomViews.OrderingView`
  - OrderingRecycleViewHolder
  - OrderingView
  - OrderingViewAdapter
- **`crc648b28e76e068c4131`** -> `AulaNative.Droid.Views.AulaValidationField`
  - AulaAdditionalDataValidationField
  - AulaMasterDataValidationField
  - AulaTextWatcher
  - AulaValidationErrorTextLayout
- **`crc648e5c3c2d7dd73a22`** -> `AulaNative.Droid.CustomViews`
  - AlignTopWrapperDrawable
  - ClickIntercepterLinearLayout
  - EmptyViewHolder
  - EventRepetitionSection
  - MediaActionOptionItem
  - NewIndicatorView
  - NotificationBadgeLayout
  - OutOfBoundCatchingGridLayoutManager
  - OutOfBoundCatchingLinearLayoutManager
  - OverlayWithTransparentCircleView
  - RecipientsGroupTextView
  - RecycleViewSectionAdapter
  - SectionAdapter`1
  - SquareLayout
  - ThreeStateSwitch
  - WarningInfoView
- **`crc64950e68cb2d0da3b5`** -> `AulaNative.Droid.CustomViews.AutoCompleteControl.ResultTable`
  - AutoCompleteControlResultAdapter
- **`crc64954218ca674dda46`** -> `AulaNative.Droid.CustomViews.GenericView`
  - GenericView
- **`crc64a0fd0c3f0e48327c`** -> `AulaNative.Droid.CustomViews.AulaBadge`
  - AulaBadge
- **`crc64abdb8e32447d89bd`** -> `AulaNative.Droid.CustomViews.Anim`
  - AnimatorListener
- **`crc64af2ffd097057c735`** -> `AulaNative.Droid.CustomViews.CustomViewPager`
  - AulaTabLayout
  - SwipeDisabledViewPager
- **`crc64ba680629999e2cd4`** -> `AulaNative.Droid.CustomViews.AutoComplete`
  - AulaAutoCompleteAdapter`1
  - AulaAutoCompleteCell
  - AulaAutoCompleteSectionCell
  - AulaAutoCompleteView
  - ExtendAutoCompleteTextView
  - FocusChangeListener
- **`crc64bd0c7aa4e298a4f2`** -> `AulaNative.Droid.CustomViews.LinkRenderingView`
  - LinKRenderingCellViewHolder
  - LinkRenderingMediaPlayerActivity
  - LinkRenderingView
  - LinkRenderingViewAdapter
- **`crc64bf0d10c1fe58d3f1`** -> `AulaNative.Droid.CustomViews.CommonViewHolders`
  - LoadingViewHolder
  - NoItemViewHolder
  - SimpleSelectableItemViewHolder
- **`crc64c23e2e7fec60456b`** -> `AulaNative.Droid.CustomViews.AulaPortalWebViewFragment`
  - AulaPortalWebView
  - AulaPortalWebView+AulaWebViewClient
  - AulaPortalWebViewActivity
  - AulaPortalWebViewFragment
  - WidgetWebViewFragment
- **`crc64c2bf81c26ee7219c`** -> `AulaNative.Droid.CustomViews.Attachment`
  - FileDowloadedBroadcastReceiver
  - FileViewActivity
  - MediaPickerActivity
  - ShowingFileAttachmentAdapter
  - ViewAttachmentViewHolder
- **`crc64c5e08b9e832e3e52`** -> `AulaNative.Droid.CustomViews.MoreOptions`
  - MoreOptionsCellHolder
  - MoreOptionsDropdownAdapter
- **`crc64c83aa1b3a9eeb70d`** -> `AulaNative.Droid.Views.AulaSelectionEditor.SelectionInputViewWithAddMoreOption`
  - AddMoreOptionItemViewHolder
  - SelectionInputViewWithAddMoreOption
  - SelectionPageWithAddMoreOptionActivity
  - SelectionPageWithAddMoreOptionAdapter
- **`crc64d5c9e17bfc1b8195`** -> `AulaNative.Droid.CustomViews.OverlayRichEditorLayout`
  - OverlayRichEditorLayout
  - OverlayRichEditorLayout+OverlayRichEditorWebViewTouchListener
- **`crc64db3f50f4396b91a5`** -> `AulaNative.Droid.CustomViews.AndroidActionSheet`
  - ContextMenuListAdapter`1
- **`crc64de339c188f2b9079`** -> `AulaNative.Droid.CustomViews.AutoCompleteSelectionList`
  - AulaAutoCompleteRecyclerViewItemMarginDecoration
  - AulaAutoCompleteSelectionListAdapter`1
  - AulaAutoCompleteSelectionListTextView
  - AulaAutoCompleteSelectionListViewHolder`1
  - BaseAulaAutoCompleteSelectionListResultAdapter`1
  - BaseAulaAutoCompleteSelectionListResultViewHolder
  - BaseAulaAutoCompleteSelectionListResultViewHolder`1
- **`crc64e01ba431b1ef99e4`** -> `AulaNative.Droid.Views.AulaImageAbbreviation`
  - AulaImageAbbreviation
  - AulaImageAbbreviation+ImageRequestListener
  - FaceCenteredCropBitmapTransformation
- **`crc64f75412af3a986ba9`** -> `AulaNative.Droid.CustomViews.AlphabetFastScrollView`
  - AlphabletScrollerAdapter
  - CharacterViewHolder
  - RecyclerViewFastScroller
  - RecyclerViewFastScroller+BubbleEventListener

### Documents

- **`crc6401be2761ec990c52`** -> `AulaNative.Droid.Activities.Document.CloudIntegration.DocumentFragment`
  - CloudStorageFragment
- **`crc6449307630a8b601a5`** -> `AulaNative.Droid.Activities.Document.SecureDocuments.Revision`
  - SecureDocumentViewRevisionActivity
  - SecureDocumentViewRevisionAdapter
  - SecureDocumentViewRevisionViewHolder
- **`crc64629431144609bb38`** -> `AulaNative.Droid.Activities.Document.SecureDocuments.Overview`
  - BaseSecureDocumentOverviewAdapter
  - BaseSecureDocumentRecycleViewHolder
  - SecureDocumentBaseFragment
  - SecureDocumentOverviewAdapter
  - SecureDocumentOverviewAdapterHeaderViewHolder
  - SecureDocumentOverviewFilteringAdapter
  - SecureDocumentOverviewFilteringCell
  - SecureDocumentOverviewFilteringFragment
  - SecureDocumentOverviewFilteringSectionCell
  - SecureDocumentOverviewFragment
  - SecureDocumentOverviewSortingFragment
  - SecureDocumentRecycleViewHolder
  - SecureDocumentRemoveAssociationFragment
  - SecureDocumentsOverviewSelectModeActivity
- **`crc6463855a261c29f242`** -> `AulaNative.Droid.Activities.Document`
  - DocumentFragment
- **`crc646df68bf978cc5370`** -> `AulaNative.Droid.Activities.Document.CloudIntegration`
  - CloudIntegrationActivity
- **`crc64755e4274aa5e2b1a`** -> `AulaNative.Droid.Activities.Document.SecureDocuments.Details`
  - SecureDocumentDetailsActivity
- **`crc6490c78c75c22ee2bd`** -> `AulaNative.Droid.Activities.Document.CommonFiles.Overview`
  - CommonFilesFilteringFragment
  - CommonFilesFragment
  - CommonFilesRecycleViewAdapter
  - CommonFilesRecycleViewHolder
- **`crc64bb8db2cc00cd9bc0`** -> `AulaNative.Droid.Activities.Document.SecureDocuments.Overview.DocumentChooseMode`
  - SecureDocumentOverviewChooseModeAdapter
  - SecureDocumentOverviewChooseModeAdapterHeaderViewHolder
  - SecureDocumentOverviewChooseModeAdapterViewHolder
- **`crc64bd13f27e022481cd`** -> `AulaNative.Droid.Activities.Document.Overview`
  - DocumentOverviewFragment
- **`crc64db7f9aea8cfbb131`** -> `AulaNative.Droid.Activities.Document.SecureDocuments.Form`
  - InternalSecureDocumentFormActivity
  - SecureDocumentImplicitSharingAdapter
  - SecureDocumentImplicitSharingViewHolder
  - SecureDocumentShareWithFormActivity
- **`crc64f21d13d69ef5a942`** -> `AulaNative.Droid.Activities.Document.SecureDocuments.CustomViews.DocumentRecipient`
  - DocumentRecipientAutoCompleteResultViewHolder

### Firebase

- **`crc645d81b9b77a3a8305`** -> `AulaNative.Droid.FireBase`
  - AulaFirebaseMessagingService

### Groups

- **`crc6426d4d8d262dda33c`** -> `AulaNative.Droid.Activities.Groups`
  - GroupDashboardActivity
  - GroupDashboardActivity+GroupDashboardPageChangeListener
  - GroupDashboardActivity+GroupDashboardPagerAdapter
  - GroupDashboardOverviewFragment
  - GroupMembershipActivity
  - GroupMembershipsListFragment
  - GroupsListAdapter
  - GroupsMenuFragment
- **`crc64f2e6333666238999`** -> `AulaNative.Droid.Activities.Groups.GroupMembershipsList`
  - GroupMembershipsAdapter
  - GroupMembershipsCellViewHolder
  - GroupMembershipsSectionViewHolder

### Messaging

- **`crc64117d19181bd40e67`** -> `AulaNative.Droid.Activities.Messages.MessageThread`
  - MessageThreadActivity
  - MessageThreadAdapter
  - MessageThreadDataAdapterObserver
  - MessageThreadFragment
  - MessageThreadFragment+ActivityResultCallback
  - MessageThreadFragment+ChildAttachStateChangeListener
- **`crc642ab953f6613e37b6`** -> `AulaNative.Droid.Activities.Messages`
  - AdvancedSearchActivity
  - AlertDialogShowListener
  - DialogAddFolder
  - MessageThreadDataAdapterObserver
  - MessageThreadViewHolder
  - MessageThreadViewHolderFoldOutCell
  - MessageThreadViewHolderFoldOutHeaderCell
  - MessageViewHolder
  - MessagesOverviewFragment
  - MessagesOverviewListAdapter
- **`crc644b585a7d6893d48b`** -> `AulaNative.Droid.Activities.Messages.MessageFolder`
  - FolderItemViewHolder
  - FolderSectionItemViewHolder
  - MessageCreateNewFolderFragment
  - MessageFolderActivity
  - MessageFolderAdapter
  - MessageFolderFragment
  - SubFolderItemViewHolder
- **`crc64982ffc6c9fa05072`** -> `AulaNative.Droid.Activities.CreateMessage`
  - CreateMessageActivity
- **`crc64e9e062cc12d35878`** -> `AulaNative.Droid.Activities.Messages.AutoReply`
  - AutoReplyActivity

### Notifications

- **`crc64a670a6203b7f22cf`** -> `AulaNative.Droid.CustomViews.NotificationTabBadge`
  - NotificationTabBadgeLayout
- **`crc64c8253ed2cd0a1e08`** -> `AulaNative.Droid.CrossPlatformServices.AndroidEmailNotificationsHandler`
  - AndroidEmailNotificationsHandler

### Onboarding & Consent

- **`crc64056db8367489a9a5`** -> `AulaNative.Droid.Activities.Onboarding.Policy`
  - AulaLinkClickMovementMethod
  - PolicyFragment
- **`crc6407ec88ddc45e0223`** -> `AulaNative.Droid.Activities.Onboarding`
  - OnboardingActivity
  - OnboardingFragmentPagerAdapter
- **`crc640f83e066fd36f625`** -> `AulaNative.Droid.Views.DataPolicyWebView`
  - DataPolicyWebView
  - DataPolicyWebView+DataPolicyWebViewClient
- **`crc646887df94713985ef`** -> `AulaNative.Droid.Activities.Onboarding.AppOnboarding`
  - AppOnboardingFragment
- **`crc64c82b547c908cd72d`** -> `AulaNative.Droid.Activities.Onboarding.Consent`
  - OnboardingConsentFragment

### Other

- **`crc640a1f4d108c17e3f1`** -> `Microsoft.Maui.ApplicationModel.DataTransfer`
  - ClipboardChangeListener
- **`crc640a8d9a12ddbf2cf2`** -> `Microsoft.Maui.Devices`
  - BatteryBroadcastReceiver
  - DeviceDisplayImplementation+Listener
  - EnergySaverBroadcastReceiver
- **`crc64396a3fe5f8138e3f`** -> `AndroidX.Browser.CustomTabs`
  - CustomTabsServiceConnectionImpl
  - KeepAliveService
- **`crc643f2b18b2570eaa5a`** -> `Microsoft.Maui.Graphics.Platform`
  - PlatformGraphicsView
- **`crc647178dc2a6e2144ea`** -> `AulaNative.Droid.Activities.ContactsList.OverView`
  - ContactListFilterAdapter
  - ContactListFilterItemViewHolder
  - ContactListFilteringGroupAdapter
  - ContactListFilteringGroupHeaderViewHolder
  - ContactListFilteringGroupInfoViewHolder
  - ContactListFilteringGroupSelectionViewHolder
  - ContactListOverviewSortingAndFilteringFragment
- **`crc6495d4f5d63cc5c882`** -> `Android.Gms.Extensions`
  - AwaitableTaskCompleteListener`1
- **`crc649881f3fa1611df58`** -> `AulaNative.Droid`
  - MainActivity
  - MainActivity+MainActivityOnPageChangedListener
- **`crc649dacca83dd48cffd`** -> `(default)`
  - MeetingParticipantSelectAdapter
- **`crc64a25b61d9f8ee364f`** -> `AndroidX.Transitions`
  - FloatArrayEvaluator
  - RectEvaluator
  - TransitionUtils
  - TransitionUtils+MatrixEvaluator
- **`crc64a7dcd4496e5253be`** -> `AulaNative.Droid.Activities.ContactsList`
  - ContactItemEmptyViewHolder
  - ContactItemViewHolder
  - ContactListAdapter
  - ContactListOverviewFragment
  - ContactsListFragment
- **`crc64ba438d8f48cf7e75`** -> `Microsoft.Maui.ApplicationModel`
  - ActivityLifecycleContextListener
  - IntermediateActivity
- **`crc64cd7981a54ec28d31`** -> `Plugin.SecureStorage`
  - ProtectedFileImplementation+StringKeyEntry
- **`crc64e53d2f592022988e`** -> `Microsoft.Maui.Networking`
  - ConnectivityBroadcastReceiver
  - ConnectivityImplementation+EssentialsNetworkCallback
- **`crc64e63ddd64dd3402b1`** -> `AulaNative.Droid.Activities`
  - TestingActivity
- **`crc64e6d0b84c6264ccdd`** -> `Plugin.Fingerprint`
  - AuthenticationHandler
- **`crc64e6f1f86f3a60c4db`** -> `AulaNative.Droid.CustomListeners`
  - DirectionTouchListener
  - ScrollDownDetector
  - ScrollStoppedDetector
- **`crc64ee5c41ee04c846c1`** -> `AulaNative.Droid.Activities.Overview`
  - OverviewFragment
  - PostsAllNotificationActivity
- **`crc64f62664462a8937a9`** -> `Microsoft.Maui.Devices.Sensors`
  - AccelerometerListener
  - BarometerListener
  - ContinuousLocationListener
  - GyroscopeListener
  - MagnetometerListener
  - OrientationSensorListener
  - SensorListener
  - SingleLocationListener
- **`crc64f723dd1a15c408a5`** -> `AulaNative.Droid.Activities.Partials`
  - AddRecipientsDialog
  - AulaToast
  - CalendarNotificationAdapter
  - CalendarNotificationViewHolder
  - CustomDialog
  - CustomListAdapter
  - EmptyViewHolder
  - EventViewHolder
  - SearchHighlightedWebView
  - TopNotificationListAdapter
  - TopNotificationViewHolder

### Posts & News

- **`crc64b07562da40b875be`** -> `AulaNative.Droid.Activities.Posts`
  - EditPostActivity
  - PostAllNotificationFragment
  - PostListViewWithFilterFragment
  - PostOverviewMultiCreationLayout
  - ViewPostActivity
- **`crc64b727de19c91cf26f`** -> `AulaNative.Droid.Activities.Posts.PostsList`
  - PostListAdapter
  - PostListViewHolder

### Search

- **`crc643b2c14d5ceefefc8`** -> `AulaNative.Droid.CustomViews.AutoCompleteControl.SearchBookables`
  - BookableSelectionCellViewHolder
- **`crc6499b40022eb512924`** -> `AulaNative.Droid.CustomViews.AutoCompleteControl.SearchRecipient`
  - SearchRecipientCellViewHolder
- **`crc64c1d40e40ba7282d6`** -> `AulaNative.Droid.Activities.Search`
  - GlobalSearchActivity
  - GlobalSearchTabAccessibilityDelegate
  - GlobalSearchViewHolder
  - GlobalSearchViewHolder`1
  - SearchAdapter
  - SearchCommonFileViewHolder
  - SearchEventViewHolder
  - SearchGroupViewHolder
  - SearchMediaViewHolder
  - SearchMessageViewHolder
  - SearchPostViewHolder
  - SearchProfileViewHolder
  - SearchSecureDocumentViewHolder
- **`crc64ea6dad1577ee648b`** -> `AulaNative.Droid.CustomViews.SearchRecipient`
  - BaseSearchRecipientAutoCompleteAdapter`1
  - SearchRecipientAutoCompleteAdapter
  - SearchRecipientAutoCompleteCell

### Settings & Menu

- **`crc64a53fe42ad64be0cd`** -> `AulaNative.Droid.Activities.Menu`
  - AbstractProfileBar
  - AulaBottomBarEditShortcutsView
  - BottomSheetAdapter`1
  - BottomSheetViewHolder`1
  - ModalBottomSheet`1
  - MoreMenuActivity
  - MoreMenuAdapter
  - MoreMenuViewHolder
  - SettingsEditShortcutsActivity
  - SettingsOverviewActivity

### User Profile

- **`crc641fcc2ac2cfa79c06`** -> `AulaNative.Droid.Activities.Onboarding.MasterData`
  - MasterDataFragment
  - MasterDataInstitutionProfileHolder
- **`crc6439eb6491da0455b9`** -> `AulaNative.Droid.Activities.PersonalReferenceData.ViewOtherMasterDataFilteringEntry`
  - ViewOtherMasterDataFilteringEntryFragment
- **`crc645f94237729b3a0f9`** -> `AulaNative.Droid.Activities.UserProfile.DeviceSettings`
  - FrontPageSettingViewHolder
  - ProfileDeviceSettingsActivity
  - ProfileDeviceSettingsAdapter
  - ProfileDeviceSettingsClearHistoryViewHolder
  - ProfileDeviceSettingsHeaderViewHolder
  - ProfileDeviceSettingsHistoryViewHolder
- **`crc647c21045b8bff1ef3`** -> `AulaNative.Droid.Activities.UserProfile.GeneralTerms`
  - ProfileGeneralTermsActivity
- **`crc6486715a2de441f74b`** -> `AulaNative.Droid.Activities.UserProfile.NotificationSettings`
  - ClearNotificationActivity
  - ClearNotificationsModulesAdapter
  - ModuleViewHolder
  - NotificationSettingsActivity
  - NotificationSettingsEditActivity
  - NotificationSettingsEditFragment
  - NotificationSettingsHeadlineViewHolder
  - NotificationSettingsItemEmailViews
  - NotificationSettingsItemViewHolder
  - NotificationSettingsListAdapter
  - NotificationSettingsNotAvailableViewHolder
  - NotificationSettingsOnboardingHeader
  - NotificationSettingsRadioButtonItem
  - NotificationSettingsRegularTextViewHolder
- **`crc6487c0dd2d74ab1a98`** -> `AulaNative.Droid.Activities.Onboarding.AdditionalMasterData`
  - OnboardingAdditionalMasterDataFragment
- **`crc648d07b834a19501a6`** -> `AulaNative.Droid.Activities.UserProfile.MasterData`
  - MasterDataActivity
  - ProfilePictureValidateAndRotateActivity
- **`crc64a8637d06d0024545`** -> `AulaNative.Droid.Activities.PersonalReferenceData.ViewOtherMasterData`
  - ViewOtherMasterDataActivity
  - ViewOtherMasterDataAdapter
  - ViewOtherMasterDataAnswerAnswerViewHolder
  - ViewOtherMasterDataFilterViewHolder
  - ViewOtherMasterDataFilteringAdapter
  - ViewOtherMasterDataFilteringFragment
  - ViewOtherMasterDataSortingFragment
- **`crc64bbda72139c7d8a39`** -> `AulaNative.Droid.Activities.UserProfile.Consents`
  - ConsentActivity
  - ConsentListAdapter
- **`crc64cbdb06e5758cf86a`** -> `AulaNative.Droid.Activities.AdditionalMasterData.RevisionHistory`
  - AdditionalDataHistoryItemViewHolder
  - AdditionalDataRevisionPageActivity
  - AdditionalDataRevisionPageAdapter
  - AdditionalDataRevisionPageInstitutionProfileViewHolder
- **`crc64e3bc9524bf401dcf`** -> `AulaNative.Droid.Activities.UserProfile`
  - ProfileActivity
  - ProfileAdapter
  - ProfileGridAdapter
  - ProfileImageViewHolder
  - ProfileOptionViewHolder
  - ProfilePrivacyAndLogOutViewHolder
  - ProfileRelationsCellViewHolder
- **`crc64ea34368975947079`** -> `AulaNative.Droid.Activities.Common.GenericProfileList`
  - GenericProfileListItemViewHolder
  - GenericProfileListSectionViewHolder
- **`crc64eda3ac7f2ee510d4`** -> `AulaNative.Droid.Activities.AdditionalMasterData`
  - AdditionalMasterDataInstitutionProfileView
  - AdditionalMasterDataSingleResponseView
  - ProfileAdditionalMasterdataActivity
- **`crc64f89139c0fb2fd8a0`** -> `AulaNative.Droid.Activities.UserProfile.Consents.ViewHolders`
  - ConsentListViewHolder
  - OnboardingTextsViewHolder
  - SingleConsentEditHolder
  - SingleConsentViewHolder

### Vacation

- **`crc644533801b3f66659a`** -> `AulaNative.Droid.Activities.ActivityList.VacationList`
  - VacationItemViewHolder
  - VacationListAdapter
